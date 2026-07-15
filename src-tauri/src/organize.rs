//! The organize / projection engine — the only code that writes to the game's mods
//! folder. Model (chosen by the user):
//!
//! * **At rest:** every mod is moved into `mods/archive/<Category>/` (subfolders the
//!   game ignores, since it only reads the flat root).
//! * **Active:** the chosen set is projected back into the flat root as **hardlinks**
//!   (zips) or dir links (unpacked mods) — same volume as the archive, so no admin /
//!   Developer Mode and no disk duplication. The game reads the flat root and loads
//!   exactly the active set.
//! * **Flatten:** everything moves back to a vanilla flat `mods/` and the archive is
//!   removed — always one step from stock.
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
        let from = root.join(&m.file_name);
        if !from.exists() {
            rep.skipped += 1;
            continue;
        }
        let dir = archive_dir(root, &m.category);
        if let Err(e) = std::fs::create_dir_all(&dir) {
            rep.errors.push(format!("{}: mkdir {}: {e}", m.tech_name, dir.display()));
            continue;
        }
        let to = dir.join(&m.file_name);
        match move_path(&from, &to) {
            Ok(()) => {
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
                } else {
                    rep.changed += 1;
                }
            }
            Err(e) => rep.errors.push(format!("{}: move: {e}", m.tech_name)),
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
                    let _ = db::set_organized_active(conn, &row.tech_name, true);
                    rep.changed += 1;
                }
                Err(e) => rep.errors.push(format!("{}: link: {e}", row.tech_name)),
            }
        } else if !want && linked {
            match remove_link(&link) {
                Ok(()) => {
                    let _ = db::set_organized_active(conn, &row.tech_name, false);
                    rep.changed += 1;
                }
                Err(e) => rep.errors.push(format!("{}: unlink: {e}", row.tech_name)),
            }
        } else {
            rep.skipped += 1;
        }
    }
    rep
}

/// Restore a vanilla flat `mods/`: remove every Silo link, move archived files back
/// to the root, delete the archive tree, and clear the manifest.
pub fn flatten(conn: &Connection, root: &Path) -> Report {
    let mut rep = Report::default();
    for row in db::load_organized(conn) {
        let link = root.join(&row.file_name);
        let src = archive_path(root, &row.category, &row.file_name);

        // Remove an active projection first so the root slot is free.
        if link.symlink_metadata().is_ok() {
            if let Err(e) = remove_link(&link) {
                rep.errors.push(format!("{}: unlink: {e}", row.tech_name));
                continue;
            }
        }
        if src.exists() {
            if let Err(e) = move_path(&src, &link) {
                rep.errors.push(format!("{}: restore: {e}", row.tech_name));
                continue;
            }
        }
        let _ = db::delete_organized(conn, &row.tech_name);
        rep.changed += 1;
    }
    // Best-effort removal of the (now-empty) archive tree.
    let _ = std::fs::remove_dir_all(root.join(ARCHIVE));
    rep
}

// ── filesystem primitives ──

/// Move a file/dir, preferring an instant same-volume rename, falling back to
/// copy+remove across volumes.
fn move_path(from: &Path, to: &Path) -> std::io::Result<()> {
    if let Some(parent) = to.parent() {
        std::fs::create_dir_all(parent)?;
    }
    match std::fs::rename(from, to) {
        Ok(()) => Ok(()),
        Err(_) => {
            if from.is_dir() {
                copy_dir_all(from, to)?;
                std::fs::remove_dir_all(from)
            } else {
                std::fs::copy(from, to)?;
                std::fs::remove_file(from)
            }
        }
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
            std::os::windows::fs::symlink_dir(src, link).or_else(|_| copy_dir_all(src, link))
        }
    } else {
        std::fs::hard_link(src, link).or_else(|_| std::fs::copy(src, link).map(|_| ()))
    }
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
