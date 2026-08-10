//! The organize / projection engine — the only code that writes to the game's mods
//! folder. Model (chosen by the user):
//!
//! * **At rest:** every mod is moved into `mods/archive/<Category>/` (subfolders the
//!   game ignores, since it only reads the flat root).
//! * **Active:** the chosen set is projected back into the flat root as **hardlinks**
//!   (zips) or dir links (unpacked mods) — same volume as the archive, so no admin /
//!   Developer Mode and no disk duplication. The game reads the flat root and loads
//!   exactly the active set.
//! * **Flatten:** everything moves back to a vanilla flat `mods/`; only *empty* archive
//!   directories are removed. Anything Silo doesn't recognize is left in place, never
//!   deleted — always one step from stock, and never a step that eats a file.
//!
//! Every move/link is recorded in the `organized` manifest so cleanup only ever
//! touches Silo-owned entries; nothing the user placed is deleted. Failures are
//! collected and reported, not panicked on.

use crate::db::{self, OrganizedRow};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

const ARCHIVE: &str = "archive";
/// Dropped inside a copy-fallback **directory** projection so we can later prove Silo
/// created it (a dir has no hardlink identity to compare). Hardlink/symlink projections
/// are identified structurally and need no marker.
const SILO_MARKER: &str = ".silo-projection";

/// A mod present in the flat root that could be organized.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModInput {
    pub tech_name: String,
    /// Basename in the flat root: `FS25_Foo.zip` (zip) or `FS25_Foo` (dir).
    pub file_name: String,
    /// "zip" | "dir"
    pub kind: String,
    pub category: String,
    pub subcategory: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlannedMove {
    pub tech_name: String,
    pub file_name: String,
    pub category: String,
    pub rel_from: String,
    pub rel_to: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub changed: usize,
    pub skipped: usize,
    pub errors: Vec<String>,
}

/// Windows-invalid path chars → underscore (spaces and `&` are fine in folder names).
fn sanitize(category: &str) -> String {
    category
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect()
}

fn archive_dir(root: &Path, category: &str) -> PathBuf {
    root.join(ARCHIVE).join(sanitize(category))
}

fn archive_path(root: &Path, category: &str, file_name: &str) -> PathBuf {
    archive_dir(root, category).join(file_name)
}

/// Dry run: which flat-root mods would move into the archive.
pub fn plan_organize(root: &Path, mods: &[ModInput]) -> Vec<PlannedMove> {
    let mut plan = Vec::new();
    for m in mods {
        let from = root.join(&m.file_name);
        // Only plan mods that are actually sitting in the flat root right now.
        if !from.exists() {
            continue;
        }
        plan.push(PlannedMove {
            tech_name: m.tech_name.clone(),
            file_name: m.file_name.clone(),
            category: m.category.clone(),
            rel_from: m.file_name.clone(),
            rel_to: format!("{ARCHIVE}/{}/{}", sanitize(&m.category), m.file_name),
        });
    }
    plan
}

/// Move flat-root mods into `archive/<Category>/` and record the manifest. Mods
/// already recorded (organized) are skipped.
pub fn apply_organize(conn: &Connection, root: &Path, mods: &[ModInput]) -> Report {
    let mut rep = Report::default();
    let already: HashSet<String> = db::load_organized(conn)
        .into_iter()
        .map(|r| r.tech_name)
        .collect();

    for m in mods {
        if already.contains(&m.tech_name) {
            rep.skipped += 1;
            continue;
        }
        // file_name drives every join below — it must be a plain basename, never a path.
        if let Err(e) = crate::paths::safe_file_name(&m.file_name) {
            rep.errors.push(format!("{}: {e}", m.tech_name));
            continue;
        }
        let from = root.join(&m.file_name);
        if !from.exists() {
            rep.skipped += 1;
            continue;
        }
        let dir = archive_dir(root, &m.category);
        if let Err(e) = std::fs::create_dir_all(&dir) {
            rep.errors
                .push(format!("{}: mkdir {}: {e}", m.tech_name, dir.display()));
            continue;
        }
        let to = dir.join(&m.file_name);
        // Manifest-first: record the row BEFORE moving the file. If the DB write fails we
        // never move, so we can't leave a file orphaned in the archive (which flatten
        // would later be unable to tell from junk). If the move then fails, roll the row
        // back. This holds under normal errors; a HARD crash between the row write and the
        // move can still leave a row with no archived file and the original still in the
        // root. That window is made non-destructive on the read side: flatten/set_active
        // never remove a root entry unless the archived copy exists behind it, so the only
        // copy is never deleted. (A journalled, startup-reconciled operation log would close
        // the window entirely — tracked as a follow-up.)
        let row = OrganizedRow {
            tech_name: m.tech_name.clone(),
            file_name: m.file_name.clone(),
            kind: m.kind.clone(),
            category: m.category.clone(),
            subcategory: m.subcategory.clone(),
            active: false,
        };
        if let Err(e) = db::upsert_organized(conn, &row) {
            rep.errors.push(format!("{}: manifest: {e}", m.tech_name));
            continue;
        }
        match move_path(&from, &to) {
            Ok(()) => rep.changed += 1,
            Err(e) => {
                // Roll back the row we just wrote — nothing was moved.
                let _ = db::delete_organized(conn, &m.tech_name);
                rep.errors.push(format!("{}: move: {e}", m.tech_name));
            }
        }
    }
    rep
}

/// Reconcile the flat-root projection to exactly `active`: link the ones that
/// should be active, remove links for the ones that shouldn't.
pub fn set_active(conn: &Connection, root: &Path, active: &HashSet<String>) -> Report {
    let mut rep = Report::default();
    for row in db::load_organized(conn) {
        let link = root.join(&row.file_name);
        let src = archive_path(root, &row.category, &row.file_name);
        let want = active.contains(&row.tech_name);
        let linked = link.symlink_metadata().is_ok();

        if want && !linked {
            match make_link(&src, &link, &row.kind) {
                Ok(()) => {
                    if let Err(e) = db::set_organized_active(conn, &row.tech_name, true) {
                        rep.errors.push(format!("{}: manifest: {e}", row.tech_name));
                    }
                    rep.changed += 1;
                }
                Err(e) => rep.errors.push(format!("{}: link: {e}", row.tech_name)),
            }
        } else if !want && linked {
            // Two guards before removing anything from the root:
            //  1. The archived copy must exist behind it — a root entry with no archive is the
            //     only copy (e.g. a crash between manifest-write and move), never delete it.
            //  2. The entry must PROVABLY be Silo's projection (our hardlink / our symlink /
            //     our marked copy). If the user swapped in their own file at that name, it is
            //     not ours to delete — leave it and say so.
            if !src.exists() {
                rep.errors.push(format!(
                    "{}: no archived copy behind the root file — kept it (won't delete the only copy)",
                    row.tech_name
                ));
                rep.skipped += 1;
            } else if !is_silo_projection(&link, &src, &row.kind) {
                rep.errors.push(format!(
                    "{}: the file in the mods folder isn't the one Silo projected (did you replace it?) — left it untouched",
                    row.tech_name
                ));
                rep.skipped += 1;
            } else {
                match remove_link(&link) {
                    Ok(()) => {
                        if let Err(e) = db::set_organized_active(conn, &row.tech_name, false) {
                            rep.errors.push(format!("{}: manifest: {e}", row.tech_name));
                        }
                        rep.changed += 1;
                    }
                    Err(e) => rep.errors.push(format!("{}: unlink: {e}", row.tech_name)),
                }
            }
        } else {
            rep.skipped += 1;
        }
    }
    rep
}

/// Adopt the file the user swapped into the mods folder as the new canonical version: back
/// up the old archived copy, promote the flat file into the archive, and (if the mod is
/// active) re-project it so it's Silo-managed again. Reversible — the old copy is kept under
/// `backups/`.
pub fn adopt_foreign(conn: &Connection, root: &Path, file_name: &str) -> Result<(), String> {
    crate::paths::safe_file_name(file_name).map_err(|e| e.to_string())?;
    let row = db::load_organized(conn)
        .into_iter()
        .find(|r| r.file_name == file_name)
        .ok_or_else(|| "Not a Silo-managed mod".to_string())?;
    let flat = root.join(file_name);
    if flat.symlink_metadata().is_err() {
        return Err("No file in the mods folder to adopt".into());
    }
    let archive = archive_path(root, &row.category, file_name);
    // Back up the old archived copy before replacing it.
    if archive.exists() {
        let bak = backup_path(root, file_name)?;
        move_path(&archive, &bak).map_err(|e| format!("backup: {e}"))?;
    } else if let Some(parent) = archive.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    // Promote the swapped-in file into the archive, then re-project it back into the root.
    move_path(&flat, &archive).map_err(|e| format!("promote: {e}"))?;
    if row.active {
        make_link(&archive, &flat, &row.kind).map_err(|e| format!("re-project: {e}"))?;
    }
    Ok(())
}

/// Restore Silo's managed copy over a file the user swapped in: back up their file, then
/// re-project the archived original (if the mod is active). Reversible — their file is kept
/// under `backups/`.
pub fn restore_projection(conn: &Connection, root: &Path, file_name: &str) -> Result<(), String> {
    crate::paths::safe_file_name(file_name).map_err(|e| e.to_string())?;
    let row = db::load_organized(conn)
        .into_iter()
        .find(|r| r.file_name == file_name)
        .ok_or_else(|| "Not a Silo-managed mod".to_string())?;
    let archive = archive_path(root, &row.category, file_name);
    if !archive.exists() {
        return Err("No archived copy to restore from".into());
    }
    let flat = root.join(file_name);
    // Back up the user's file, then free the slot.
    if flat.symlink_metadata().is_ok() {
        let bak = backup_path(root, file_name)?;
        move_path(&flat, &bak).map_err(|e| format!("backup: {e}"))?;
    }
    if row.active {
        make_link(&archive, &flat, &row.kind).map_err(|e| format!("re-project: {e}"))?;
    }
    Ok(())
}

/// A timestamped, non-clobbering location under `backups/` for a file we're about to replace.
fn backup_path(root: &Path, file_name: &str) -> Result<PathBuf, String> {
    let dir = root.join("backups");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    Ok(dir.join(format!("{file_name}.{secs}.bak")))
}

/// Restore a vanilla flat `mods/`: remove every Silo link, move archived files back
/// to the root, remove only empty archive directories (never an unrecognized file), and
/// clear the manifest.
pub fn flatten(conn: &Connection, root: &Path) -> Report {
    let mut rep = Report::default();
    for row in db::load_organized(conn) {
        let link = root.join(&row.file_name);
        let src = archive_path(root, &row.category, &row.file_name);

        if src.exists() {
            let occupied = link.symlink_metadata().is_ok();
            // If a file occupies the root slot but it isn't Silo's projection (the user swapped
            // their own build in), don't remove it and don't overwrite it with the archived
            // copy — leave both in place and report the anomaly for the user to resolve.
            if occupied && !is_silo_projection(&link, &src, &row.kind) {
                rep.errors.push(format!(
                    "{}: the mods-folder file isn't Silo's projection — left it and the archived copy untouched",
                    row.tech_name
                ));
                rep.skipped += 1;
                continue;
            }
            // Normal restore: drop our projection to free the slot, then move the archived
            // original back into the root.
            if occupied {
                if let Err(e) = remove_link(&link) {
                    rep.errors.push(format!("{}: unlink: {e}", row.tech_name));
                    continue;
                }
            }
            if let Err(e) = move_path(&src, &link) {
                rep.errors.push(format!("{}: restore: {e}", row.tech_name));
                continue;
            }
        } else if link.exists() {
            // A file sits in the root slot but there is NO archived copy behind it. That is
            // the original, not a projection (a projection always has an archive) — this is
            // exactly the state a crash between the manifest write and the move leaves. Never
            // delete it: keep the file, drop the stale manifest row.
            rep.errors.push(format!(
                "{}: no archived copy — kept the file in the mods root (not deleted)",
                row.tech_name
            ));
        }
        // else: neither archive nor root file — nothing to restore; just clear the row.
        if let Err(e) = db::delete_organized(conn, &row.tech_name) {
            rep.errors.push(format!("{}: manifest: {e}", row.tech_name));
            continue;
        }
        rep.changed += 1;
    }
    // NEVER `remove_dir_all` the archive — that would destroy any file we don't have a
    // manifest row for (e.g. a mod whose manifest write failed on a crash). Remove only
    // provably-empty directories, and surface anything left behind untouched.
    match cleanup_empty_archive(&root.join(ARCHIVE)) {
        Ok(leftover) => {
            for p in leftover {
                let name = p.file_name().map(|n| n.to_string_lossy().into_owned());
                rep.errors.push(format!(
                    "kept an unrecognized file in archive/ (NOT deleted): {}",
                    name.unwrap_or_else(|| p.display().to_string())
                ));
            }
        }
        Err(e) => rep.errors.push(format!("archive cleanup: {e}")),
    }
    rep
}

/// Remove empty directories under `archive` bottom-up. NEVER deletes a file or a
/// non-empty directory — any file the manifest didn't restore is left in place and
/// returned so the caller can tell the user. This is the guard against the flatten
/// step eating an untracked mod.
fn cleanup_empty_archive(archive: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut leftover = Vec::new();
    if !archive.exists() {
        return Ok(leftover);
    }
    // Returns true if `dir` is empty once processing finishes.
    fn walk(dir: &Path, leftover: &mut Vec<PathBuf>) -> std::io::Result<bool> {
        let mut empty = true;
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            // Use symlink semantics: never follow/recurse a link.
            if entry.file_type()?.is_dir() {
                if walk(&path, leftover)? {
                    let _ = std::fs::remove_dir(&path);
                } else {
                    empty = false;
                }
            } else {
                leftover.push(path);
                empty = false;
            }
        }
        Ok(empty)
    }
    if walk(archive, &mut leftover)? {
        let _ = std::fs::remove_dir(archive);
    }
    Ok(leftover)
}

// ── filesystem primitives ──

/// A hidden temp sibling of `to`, used for the copy-and-swap fallback.
fn tmp_sibling(to: &Path) -> PathBuf {
    let name = to
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "x".into());
    to.with_file_name(format!(".{name}.silo-tmp"))
}

/// Remove a path whatever it is (file/dir/link), treating "not found" as success.
fn remove_path(p: &Path) -> std::io::Result<()> {
    match std::fs::symlink_metadata(p) {
        Ok(m) if m.is_dir() => std::fs::remove_dir_all(p),
        Ok(_) => std::fs::remove_file(p),
        Err(_) => Ok(()),
    }
}

/// Move a file/dir, preferring an instant same-volume rename. On any rename failure
/// (typically a cross-volume move) fall back SAFELY: copy to a temporary sibling of the
/// destination, rename that into place, and only then remove the source — so a failed or
/// partial copy never leaves a half-written file at `to`, and `from` is never lost until
/// the destination is complete.
fn move_path(from: &Path, to: &Path) -> std::io::Result<()> {
    if let Some(parent) = to.parent() {
        std::fs::create_dir_all(parent)?;
    }
    if std::fs::rename(from, to).is_ok() {
        return Ok(());
    }

    let tmp = tmp_sibling(to);
    let _ = remove_path(&tmp); // clear any stale temp from a prior aborted run
    let copied = if from.is_dir() {
        copy_dir_all(from, &tmp)
    } else {
        std::fs::copy(from, &tmp).map(|_| ())
    };
    if let Err(e) = copied {
        let _ = remove_path(&tmp);
        return Err(e);
    }
    if let Err(e) = std::fs::rename(&tmp, to) {
        let _ = remove_path(&tmp);
        return Err(e);
    }
    if from.is_dir() {
        std::fs::remove_dir_all(from)
    } else {
        std::fs::remove_file(from)
    }
}

/// Project `src` into the flat root at `link`. Files → hardlink (no privilege,
/// same volume, no disk duplication); dirs → OS symlink, falling back to copy.
fn make_link(src: &Path, link: &Path, kind: &str) -> std::io::Result<()> {
    if kind == "dir" {
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(src, link)
        }
        #[cfg(windows)]
        {
            std::os::windows::fs::symlink_dir(src, link).or_else(|_| copy_dir_projection(src, link))
        }
    } else {
        std::fs::hard_link(src, link).or_else(|_| std::fs::copy(src, link).map(|_| ()))
    }
}

/// Copy a directory as a projection and drop the ownership marker, so a later flatten /
/// deactivate can prove Silo created it (a copied dir has no link identity to compare).
#[cfg(windows)]
fn copy_dir_projection(src: &Path, dst: &Path) -> std::io::Result<()> {
    copy_dir_all(src, dst)?;
    std::fs::write(dst.join(SILO_MARKER), b"silo\n")
}

/// True only if `link` in the flat root is provably the projection Silo created for archived
/// `src` — so removing it can't destroy a file the user put there themselves. Symlink /
/// junction resolves to `src`; a zip hardlink shares `src`'s file id; a cross-volume copy is
/// byte-identical to `src`; a copied dir carries our marker. Anything else is NOT ours.
fn is_silo_projection(link: &Path, src: &Path, kind: &str) -> bool {
    let Ok(meta) = link.symlink_metadata() else {
        return false;
    };
    let ft = meta.file_type();

    if ft.is_symlink() {
        // Our dir symlink / Windows junction resolves to the archived source.
        return matches!(
            (std::fs::canonicalize(link), std::fs::canonicalize(src)),
            (Ok(a), Ok(b)) if a == b
        );
    }

    if kind == "dir" || ft.is_dir() {
        // Copy-fallback directory projection carries our marker; a user's own folder won't.
        return link.join(SILO_MARKER).is_file();
    }

    // Regular file (zip). Same underlying file id → our hardlink (cheap, the common case).
    if let (Ok(a), Ok(b)) = (
        same_file::Handle::from_path(link),
        same_file::Handle::from_path(src),
    ) {
        if a == b {
            return true;
        }
    }
    // Distinct files: a cross-volume copy projection is byte-identical to the archive; a
    // user's replacement is not. Cheap size gate first, then a content hash.
    let (Ok(lm), Ok(sm)) = (std::fs::metadata(link), std::fs::metadata(src)) else {
        return false;
    };
    if lm.len() != sm.len() {
        return false;
    }
    matches!(
        (
            crate::provenance::sha256_file(link),
            crate::provenance::sha256_file(src),
        ),
        (Ok(a), Ok(b)) if a == b
    )
}

/// A file in the flat root occupying an organized mod's managed name that ISN'T Silo's
/// projection — something the user (or another tool) placed there. Silo never deletes it, but
/// the user should know their intended mod isn't what the game will load from that name.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForeignFile {
    pub tech_name: String,
    pub file_name: String,
    pub kind: String, // "zip" | "dir"
    /// Version of the file currently in the mods folder (what "adopt" would keep).
    pub flat_version: Option<String>,
    /// Version of Silo's managed/archived copy (what "restore" would put back).
    pub managed_version: Option<String>,
}

/// Best-effort mod version from a mod's `modDesc.xml`, for showing which build is which.
fn mod_version(path: &Path, kind: &str) -> Option<String> {
    crate::scan::read_moddesc_xml(path, kind)
        .ok()
        .and_then(|xml| crate::moddesc::parse(&xml).version)
}

/// Scan the flat root for foreign/mismatched files sitting at organized mods' names. Cheap:
/// parked mods leave the flat root empty, so it only identity-checks the handful of entries
/// actually present against their archived source. Reuses [`is_silo_projection`], so a real
/// hardlink/symlink/copy projection is never flagged — only a file Silo can't prove it created.
pub fn detect_foreign_projections(root: &Path) -> Vec<ForeignFile> {
    let archive = root.join(ARCHIVE);
    if !archive.is_dir() {
        return Vec::new();
    }
    // Map each organized mod's file name → its archived path (archive/<Category>/<name>).
    let mut archived: std::collections::HashMap<std::ffi::OsString, PathBuf> =
        std::collections::HashMap::new();
    if let Ok(cats) = std::fs::read_dir(&archive) {
        for cat in cats.flatten() {
            if cat.path().is_dir() {
                if let Ok(entries) = std::fs::read_dir(cat.path()) {
                    for m in entries.flatten() {
                        archived.insert(m.file_name(), m.path());
                    }
                }
            }
        }
    }

    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(root) else {
        return out;
    };
    for e in entries.flatten() {
        let name = e.file_name();
        if name == ARCHIVE || name == "backups" {
            continue;
        }
        // Only a name that maps to an organized mod counts — a plain loose zip with no archived
        // counterpart is an ordinary "unorganized" mod (Organize handles it), not a mismatch.
        let Some(src) = archived.get(&name) else {
            continue;
        };
        let link = e.path();
        let kind = if src.is_dir() { "dir" } else { "zip" };
        if !is_silo_projection(&link, src, kind) {
            let file_name = name.to_string_lossy().into_owned();
            let tech_name = file_name
                .strip_suffix(".zip")
                .unwrap_or(&file_name)
                .to_string();
            out.push(ForeignFile {
                tech_name,
                file_name,
                kind: kind.to_string(),
                flat_version: mod_version(&link, kind),
                managed_version: mod_version(src, kind),
            });
        }
    }
    out
}

/// Remove a projected entry (hardlink, symlink, junction, or copy) without
/// touching the archived original.
fn remove_link(link: &Path) -> std::io::Result<()> {
    let meta = link.symlink_metadata()?;
    let ft = meta.file_type();
    if ft.is_symlink() {
        // A dir symlink/junction must be removed with remove_dir on Windows.
        if link.metadata().map(|m| m.is_dir()).unwrap_or(false) {
            std::fs::remove_dir(link)
        } else {
            std::fs::remove_file(link)
        }
    } else if ft.is_dir() {
        std::fs::remove_dir_all(link)
    } else {
        std::fs::remove_file(link)
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let to = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &to)?;
        } else {
            std::fs::copy(entry.path(), &to)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_silo_projection_distinguishes_ours_from_the_users() {
        let base = std::env::temp_dir().join(format!("silo_proj_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).unwrap();
        let src = base.join("archived_FS25_Foo.zip");
        std::fs::write(&src, b"PK\x03\x04 canonical bytes").unwrap();
        let link = base.join("FS25_Foo.zip");

        // Our hardlink projection → ours.
        std::fs::hard_link(&src, &link).unwrap();
        assert!(is_silo_projection(&link, &src, "zip"), "hardlink is ours");
        std::fs::remove_file(&link).unwrap();

        // A byte-identical copy (cross-volume fallback) → ours.
        std::fs::copy(&src, &link).unwrap();
        assert!(
            is_silo_projection(&link, &src, "zip"),
            "identical copy is ours"
        );
        std::fs::remove_file(&link).unwrap();

        // The user's own DIFFERENT build at the same name → NOT ours (must be preserved).
        std::fs::write(&link, b"PK\x03\x04 the user's different, longer build").unwrap();
        assert!(
            !is_silo_projection(&link, &src, "zip"),
            "a user's replacement must not read as Silo's"
        );

        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn is_silo_projection_dir_requires_marker() {
        let base = std::env::temp_dir().join(format!("silo_projdir_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).unwrap();
        let src = base.join("archived_dir");
        std::fs::create_dir_all(&src).unwrap();
        let link = base.join("FS25_FooDir");
        std::fs::create_dir_all(&link).unwrap();
        std::fs::write(link.join("modDesc.xml"), b"<x/>").unwrap();

        // A plain user directory at the projected name → NOT ours.
        assert!(!is_silo_projection(&link, &src, "dir"));
        // With Silo's ownership marker → ours.
        std::fs::write(link.join(SILO_MARKER), b"silo\n").unwrap();
        assert!(is_silo_projection(&link, &src, "dir"));

        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn cleanup_never_deletes_unknown_files() {
        let base = std::env::temp_dir().join(format!("silo_flatten_a_{}", std::process::id()));
        let archive = base.join(ARCHIVE);
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(archive.join("Maps")).unwrap(); // empty → should be removed
        std::fs::create_dir_all(archive.join("Vehicles")).unwrap();
        // An orphan the manifest doesn't know about (e.g. a move whose DB write crashed).
        let orphan = archive.join("Vehicles").join("orphan.zip");
        std::fs::write(&orphan, b"PK\x03\x04").unwrap();

        let leftover = cleanup_empty_archive(&archive).unwrap();

        assert!(orphan.exists(), "orphan file must NOT be deleted");
        assert_eq!(leftover.len(), 1);
        assert!(
            !archive.join("Maps").exists(),
            "empty dir should be removed"
        );
        assert!(archive.join("Vehicles").exists(), "non-empty dir kept");
        assert!(
            archive.exists(),
            "archive kept because it still holds the orphan"
        );

        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn flatten_never_deletes_the_original_when_no_archive_copy_exists() {
        // Simulate the crash window: a manifest row was written but the move never happened,
        // so the ORIGINAL still sits in the flat root and there is no archived copy. Flatten
        // must keep the file and only clear the stale row — never delete the only copy.
        let base = std::env::temp_dir().join(format!("silo_crashwin_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).unwrap();
        let conn = db::open(&base.join("silo.db")).unwrap();

        let row = OrganizedRow {
            tech_name: "FS25_Foo".into(),
            file_name: "FS25_Foo.zip".into(),
            kind: "zip".into(),
            category: "Vehicles".into(),
            subcategory: None,
            active: false,
        };
        db::upsert_organized(&conn, &row).unwrap();
        let original = base.join("FS25_Foo.zip");
        std::fs::write(&original, b"PK\x03\x04 the user's only copy").unwrap();
        // No archive/Vehicles/FS25_Foo.zip exists.

        let rep = flatten(&conn, &base);

        assert!(original.exists(), "the only copy must NOT be deleted");
        assert!(
            db::load_organized(&conn).is_empty(),
            "the stale manifest row should be cleared"
        );
        assert!(
            rep.errors.iter().any(|e| e.contains("kept the file")),
            "the kept-file case should be reported, got: {:?}",
            rep.errors
        );

        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn cleanup_removes_a_fully_empty_archive() {
        let base = std::env::temp_dir().join(format!("silo_flatten_b_{}", std::process::id()));
        let archive = base.join(ARCHIVE);
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(archive.join("A").join("B")).unwrap();

        let leftover = cleanup_empty_archive(&archive).unwrap();

        assert!(leftover.is_empty());
        assert!(!archive.exists(), "a fully-empty archive tree is removed");

        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn detect_foreign_flags_only_non_projections() {
        let root = std::env::temp_dir().join(format!("silo_foreign_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let cat = root.join("archive").join("Vehicles");
        std::fs::create_dir_all(&cat).unwrap();
        let src = cat.join("FS25_Foo.zip");
        std::fs::write(&src, b"the real archived mod bytes").unwrap();

        // Nothing in the flat root → nothing flagged.
        assert!(detect_foreign_projections(&root).is_empty());

        // A genuine hardlink projection → NOT flagged (it's provably ours).
        let link = root.join("FS25_Foo.zip");
        std::fs::hard_link(&src, &link).unwrap();
        assert!(
            detect_foreign_projections(&root).is_empty(),
            "a real hardlink projection must not be flagged"
        );
        std::fs::remove_file(&link).unwrap();

        // A DIFFERENT file at that managed name → flagged as foreign.
        std::fs::write(&link, b"the user's own, different build").unwrap();
        let found = detect_foreign_projections(&root);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].tech_name, "FS25_Foo");
        assert_eq!(found[0].file_name, "FS25_Foo.zip");

        // A loose zip with NO archived counterpart is an ordinary unorganized mod, not foreign.
        std::fs::write(root.join("FS25_Unrelated.zip"), b"loose").unwrap();
        assert_eq!(detect_foreign_projections(&root).len(), 1);

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn adopt_promotes_the_mods_folder_file_and_backs_up_the_managed_copy() {
        let base = std::env::temp_dir().join(format!("silo_adopt_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).unwrap();
        let conn = db::open(&base.join("silo.db")).unwrap();
        let cat = base.join(ARCHIVE).join("Vehicles");
        std::fs::create_dir_all(&cat).unwrap();
        std::fs::write(cat.join("FS25_Foo.zip"), b"OLD archived build").unwrap();
        db::upsert_organized(
            &conn,
            &OrganizedRow {
                tech_name: "FS25_Foo".into(),
                file_name: "FS25_Foo.zip".into(),
                kind: "zip".into(),
                category: "Vehicles".into(),
                subcategory: None,
                active: false,
            },
        )
        .unwrap();
        std::fs::write(base.join("FS25_Foo.zip"), b"NEWER mods-folder build").unwrap();

        adopt_foreign(&conn, &base, "FS25_Foo.zip").unwrap();

        assert_eq!(
            std::fs::read(cat.join("FS25_Foo.zip")).unwrap(),
            b"NEWER mods-folder build",
            "the mods-folder build becomes the managed copy"
        );
        assert!(
            !base.join("FS25_Foo.zip").exists(),
            "parked → the flat slot is emptied"
        );
        let baks: Vec<_> = std::fs::read_dir(base.join("backups"))
            .unwrap()
            .flatten()
            .collect();
        assert_eq!(baks.len(), 1, "the old managed copy is backed up");
        assert_eq!(std::fs::read(baks[0].path()).unwrap(), b"OLD archived build");

        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn restore_puts_silos_copy_back_and_backs_up_the_swap() {
        let base = std::env::temp_dir().join(format!("silo_restore_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).unwrap();
        let conn = db::open(&base.join("silo.db")).unwrap();
        let cat = base.join(ARCHIVE).join("Vehicles");
        std::fs::create_dir_all(&cat).unwrap();
        std::fs::write(cat.join("FS25_Foo.zip"), b"MANAGED build").unwrap();
        db::upsert_organized(
            &conn,
            &OrganizedRow {
                tech_name: "FS25_Foo".into(),
                file_name: "FS25_Foo.zip".into(),
                kind: "zip".into(),
                category: "Vehicles".into(),
                subcategory: None,
                active: true,
            },
        )
        .unwrap();
        std::fs::write(base.join("FS25_Foo.zip"), b"the user's swapped-in build").unwrap();

        restore_projection(&conn, &base, "FS25_Foo.zip").unwrap();

        assert_eq!(
            std::fs::read(base.join("FS25_Foo.zip")).unwrap(),
            b"MANAGED build",
            "the flat slot is re-projected from the managed copy"
        );
        let baks: Vec<_> = std::fs::read_dir(base.join("backups"))
            .unwrap()
            .flatten()
            .collect();
        assert_eq!(baks.len(), 1, "the swapped-in build is backed up");
        assert_eq!(
            std::fs::read(baks[0].path()).unwrap(),
            b"the user's swapped-in build"
        );

        let _ = std::fs::remove_dir_all(&base);
    }
}
