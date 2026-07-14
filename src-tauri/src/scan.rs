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

    pub dependencies: Vec<String>,
    pub script_count: usize,
    pub registration_count: usize,
    pub unique_type: Option<String>,
    pub store_item_count: u32,
    pub mp_supported: bool,

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

/// Enumerate mod candidates in a single flat root (the game does not recurse).
fn collect_candidates(root: &Path) -> Vec<Candidate> {
    let mut out = Vec::new();
    let Ok(rd) = fs::read_dir(root) else {
        return out;
    };
    for entry in rd.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        let file_type = entry.file_type();

        if let Ok(ft) = file_type {
            if ft.is_dir() {
                if path.join("modDesc.xml").is_file() {
                    out.push(Candidate {
                        tech_name: name,
                        path,
                        kind: "dir",
                    });
                }
            } else if ft.is_file() && name.to_lowercase().ends_with(".zip") {
                // Strip the .zip suffix case-insensitively to get the tech name.
                let tech_name = name[..name.len() - 4].to_string();
                out.push(Candidate {
                    tech_name,
                    path,
                    kind: "zip",
                });
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
        dependencies: Vec::new(),
        script_count: 0,
        registration_count: 0,
        unique_type: None,
        store_item_count: 0,
        mp_supported: false,
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
            entry.title = md.title.or_else(|| Some(c.tech_name.clone()));
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

    let mut candidates: Vec<Candidate> = Vec::new();
    for root in &roots {
        candidates.extend(collect_candidates(root));
    }
    let total = candidates.len();
    progress(0, total);

    let done = AtomicUsize::new(0);
    let step = (total / 100).max(10);

    let pairs: Vec<(ModEntry, bool)> = candidates
        .par_iter()
        .map(|c| {
            let resolved = resolve_entry(c, cache);
            let n = done.fetch_add(1, Ordering::Relaxed) + 1;
            if n % step == 0 || n == total {
                progress(n, total);
            }
            resolved
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
