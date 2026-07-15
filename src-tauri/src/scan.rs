//! Library scan: walk the mod root(s), read each mod's `modDesc.xml`, parse it,
//! and return structured entries. All heavy work runs here on a rayon pool — the
//! UI thread never touches a zip. We read ONLY the `modDesc.xml` entry from each
//! archive (the zip central directory + one small entry), so even 100 MB+ map
//! mods scan cheaply. Content hashing and icon decode are deliberately deferred.

use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::db::CacheEntry;
use crate::moddesc;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModEntry {
    /// Tech name = the zip/dir basename (mod identity the game & deps use).
    pub tech_name: String,
    pub path: String,
    /// "zip" | "dir"
    pub kind: String,
    pub size: u64,
    pub mtime_ms: u64,

    pub title: Option<String>,
    pub author: Option<String>,
    pub version: Option<String>,
    pub desc_version: Option<i64>,
    /// Path to the mod's icon inside the archive/dir (for lazy thumbnail loading).
    pub icon_filename: Option<String>,

    pub is_map: bool,
    pub map_title: Option<String>,
    /// Best-effort category folder bucket (see `category` module).
    pub category: String,
    /// Optional second-level bucket (e.g. Tractors › Medium).
    pub subcategory: Option<String>,

    pub dependencies: Vec<String>,
    pub script_count: usize,
    pub registration_count: usize,
    pub unique_type: Option<String>,
    pub store_item_count: u32,
    pub mp_supported: bool,

    /// True when the mod lives in `mods/archive/<Category>/` (Silo-managed).
    pub organized: bool,
    /// True when the mod is present in the flat root (i.e. the game loads it):
    /// vanilla mods, and organized mods currently projected as links.
    pub active: bool,

    /// The game IGNORES mods whose name starts with a digit — a silent footgun.
    pub ignored_digit_prefix: bool,
    /// Populated when the mod couldn't be read/parsed (still listed, flagged).
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanResult {
    pub mods: Vec<ModEntry>,
    pub roots: Vec<String>,
    pub took_ms: u128,
    pub total: usize,
}

/// Result of a cached scan: the full list plus which paths were freshly parsed
/// (cache misses) so the caller can persist just those.
pub struct ScanOutput {
    pub result: ScanResult,
    pub fresh_paths: HashSet<String>,
}

struct Candidate {
    path: PathBuf,
    tech_name: String,
    kind: &'static str,
}

/// Build a Candidate from a directory entry (a `.zip` file or an unpacked mod dir),
/// or None if it isn't a mod.
fn candidate_from(path: PathBuf, name: &str, ft: std::fs::FileType) -> Option<Candidate> {
    if ft.is_dir() {
        if path.join("modDesc.xml").is_file() {
            return Some(Candidate { tech_name: name.to_string(), path, kind: "dir" });
        }
    } else if ft.is_file() && name.to_lowercase().ends_with(".zip") {
        let tech_name = name[..name.len() - 4].to_string();
        return Some(Candidate { tech_name, path, kind: "zip" });
    }
    None
}

/// Mods in the flat root (what the game reads) — excluding Silo's `archive/` and
/// any `backups/` folder.
fn collect_root_candidates(root: &Path) -> Vec<Candidate> {
    let mut out = Vec::new();
    let Ok(rd) = fs::read_dir(root) else {
        return out;
    };
    for entry in rd.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        let lname = name.to_lowercase();
        if lname == "archive" || lname == "backups" {
            continue;
        }
        if let Ok(ft) = entry.file_type() {
            if let Some(c) = candidate_from(entry.path(), &name, ft) {
                out.push(c);
            }
        }
    }
    out
}

/// Mods parked in `mods/archive/<Category>/` (Silo-managed, one level of folders).
fn collect_archive_candidates(root: &Path) -> Vec<Candidate> {
    let mut out = Vec::new();
    let archive = root.join("archive");
    let Ok(cats) = fs::read_dir(&archive) else {
        return out;
    };
    for cat in cats.flatten() {
        if !cat.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            continue;
        }
        let Ok(mods) = fs::read_dir(cat.path()) else {
            continue;
        };
        for entry in mods.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if let Ok(ft) = entry.file_type() {
                if let Some(c) = candidate_from(entry.path(), &name, ft) {
                    out.push(c);
                }
            }
        }
    }
    out
}

/// Read a mod's `modDesc.xml` text, whether it's a `.zip` or an unpacked dir.
pub fn read_moddesc_xml(mod_path: &Path, kind: &str) -> Result<String, String> {
    match kind {
        "zip" => read_moddesc_from_zip(mod_path),
        _ => read_moddesc_from_dir(mod_path),
    }
}

fn read_moddesc_from_zip(path: &Path) -> Result<String, String> {
    let file = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
    // modDesc.xml lives at the archive root per FS conventions.
    let mut f = archive
        .by_name("modDesc.xml")
        .map_err(|_| "modDesc.xml not found in archive".to_string())?;
    let mut s = String::new();
    f.read_to_string(&mut s).map_err(|e| e.to_string())?;
    Ok(s)
}

fn read_moddesc_from_dir(path: &Path) -> Result<String, String> {
    fs::read_to_string(path.join("modDesc.xml")).map_err(|e| e.to_string())
}

fn build_entry(c: &Candidate) -> ModEntry {
    let meta = fs::metadata(&c.path).ok();
    let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
    let mtime_ms = meta
        .as_ref()
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);

    let ignored_digit_prefix = c
        .tech_name
        .chars()
        .next()
        .map(|ch| ch.is_ascii_digit())
        .unwrap_or(false);

    let mut entry = ModEntry {
        tech_name: c.tech_name.clone(),
        path: c.path.to_string_lossy().into_owned(),
        kind: c.kind.to_string(),
        size,
        mtime_ms,
        title: None,
        author: None,
        version: None,
        desc_version: None,
        icon_filename: None,
        is_map: false,
        map_title: None,
        category: "Other".to_string(),
        subcategory: None,
        dependencies: Vec::new(),
        script_count: 0,
        registration_count: 0,
        unique_type: None,
        store_item_count: 0,
        mp_supported: false,
        organized: false,
        active: false,
        ignored_digit_prefix,
        error: None,
    };

    let xml = match c.kind {
        "zip" => read_moddesc_from_zip(&c.path),
        _ => read_moddesc_from_dir(&c.path),
    };

    match xml {
        Ok(xml) => {
            let md = moddesc::parse(&xml);
            // Compute category while `md` is still whole (before we move fields out).
            // Read the authoritative FS store category when the mod has store items.
            let store_cat = if md.store_item_files.is_empty() {
                None
            } else {
                crate::store::first_store_category(&c.path, c.kind, &md.store_item_files)
            };
            let title = md.title.clone().or_else(|| Some(c.tech_name.clone()));
            let (category, subcategory) =
                crate::category::categorize(&md, store_cat.as_deref(), &c.tech_name, title.as_deref());
            entry.category = category;
            entry.subcategory = subcategory;
            entry.title = title;
            entry.author = md.author;
            entry.version = md.version;
            entry.desc_version = md.desc_version;
            entry.icon_filename = md.icon_filename;
            entry.is_map = md.is_map;
            entry.map_title = md.map_title;
            entry.dependencies = md.dependencies;
            entry.script_count = md.scripts.len();
            entry.registration_count = md.registrations.len();
            entry.unique_type = md.unique_type;
            entry.store_item_count = md.store_item_count;
            entry.mp_supported = md.mp_supported;
        }
        Err(e) => {
            entry.title = Some(c.tech_name.clone());
            entry.error = Some(e);
        }
    }

    entry
}

/// Resolve one candidate: reuse the cached entry when mtime+size are unchanged
/// (no archive open), otherwise parse fresh. Returns (entry, was_freshly_parsed).
fn resolve_entry(c: &Candidate, cache: &HashMap<String, CacheEntry>) -> (ModEntry, bool) {
    let meta = fs::metadata(&c.path).ok();
    let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
    let mtime_ms = meta
        .as_ref()
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);

    if let Some(ce) = cache.get(c.path.to_string_lossy().as_ref()) {
        if ce.mtime_ms == mtime_ms && ce.size == size {
            // A schema change that fails deserialization simply falls through to a
            // fresh parse — we never trust a partial decode.
            if let Ok(entry) = serde_json::from_str::<ModEntry>(&ce.json) {
                return (entry, false);
            }
        }
    }
    (build_entry(c), true)
}

/// Core scan: walk the roots, resolving each candidate from `cache` when possible
/// and parsing the rest in parallel. Reports progress via a callback (invoked from
/// many rayon threads, hence Sync). No Tauri dependency — testable / CLI-reusable.
pub fn scan_cached<F>(
    roots: Vec<PathBuf>,
    cache: &HashMap<String, CacheEntry>,
    progress: F,
) -> ScanOutput
where
    F: Fn(usize, usize) + Sync + Send,
{
    let started = std::time::Instant::now();

    let mut root_cands: Vec<Candidate> = Vec::new();
    let mut archive_cands: Vec<Candidate> = Vec::new();
    for root in &roots {
        root_cands.extend(collect_root_candidates(root));
        archive_cands.extend(collect_archive_candidates(root));
    }

    // Tech-names present in the archive (organized) and in the flat root (loaded).
    let archived: HashSet<String> = archive_cands.iter().map(|c| c.tech_name.clone()).collect();
    let in_root: HashSet<String> = root_cands.iter().map(|c| c.tech_name.clone()).collect();

    // Parse every archived mod, plus flat-root mods that AREN'T organized (vanilla).
    // A flat-root entry whose tech-name is already in the archive is just an active
    // projection (hardlink) — we don't re-parse it, only flag the archive entry.
    let to_parse: Vec<(&Candidate, bool)> = archive_cands
        .iter()
        .map(|c| (c, true))
        .chain(
            root_cands
                .iter()
                .filter(|c| !archived.contains(&c.tech_name))
                .map(|c| (c, false)),
        )
        .collect();

    let total = to_parse.len();
    progress(0, total);

    let done = AtomicUsize::new(0);
    let step = (total / 100).max(10);

    let pairs: Vec<(ModEntry, bool)> = to_parse
        .par_iter()
        .map(|(c, organized)| {
            let (mut entry, fresh) = resolve_entry(c, cache);
            entry.organized = *organized;
            // Organized mods are "active" only when also linked into the flat root;
            // vanilla (unorganized) mods sit in the root, so they're active.
            entry.active = if *organized {
                in_root.contains(&entry.tech_name)
            } else {
                true
            };
            let n = done.fetch_add(1, Ordering::Relaxed) + 1;
            if n % step == 0 || n == total {
                progress(n, total);
            }
            (entry, fresh)
        })
        .collect();

    let mut fresh_paths = HashSet::new();
    let mut mods = Vec::with_capacity(pairs.len());
    for (entry, fresh) in pairs {
        if fresh {
            fresh_paths.insert(entry.path.clone());
        }
        mods.push(entry);
    }

    mods.sort_by(|a, b| {
        let ta = a.title.as_deref().unwrap_or(&a.tech_name).to_lowercase();
        let tb = b.title.as_deref().unwrap_or(&b.tech_name).to_lowercase();
        ta.cmp(&tb)
    });

    progress(total, total);

    ScanOutput {
        result: ScanResult {
            mods,
            roots: roots.iter().map(|p| p.to_string_lossy().into_owned()).collect(),
            took_ms: started.elapsed().as_millis(),
            total,
        },
        fresh_paths,
    }
}

/// Convenience wrapper with no cache — used by the example harness and tests.
pub fn scan_with<F>(roots: Vec<PathBuf>, progress: F) -> ScanResult
where
    F: Fn(usize, usize) + Sync + Send,
{
    scan_cached(roots, &HashMap::new(), progress).result
}
