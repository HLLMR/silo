//! Silo core — Tauri command surface. All heavy logic lives in sibling modules so
//! it stays unit-testable (and reusable by a future CLI) without a running app.

pub mod category;
pub mod db;
pub mod fsgame;
pub mod icons;
pub mod moddesc;
pub mod scan;
pub mod store;

use std::collections::HashSet;
use std::path::PathBuf;
use tauri::{Emitter, Manager};

#[derive(Clone, serde::Serialize)]
struct Progress {
    done: usize,
    total: usize,
}

fn db_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map(|d| d.join("silo.db"))
        .map_err(|e| e.to_string())
}

/// Return the auto-detected default mod root(s) as strings for the UI.
#[tauri::command]
fn default_mods_paths() -> Vec<String> {
    fsgame::default_mods_paths()
        .into_iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect()
}

/// Scan the given roots (or the auto-detected default when omitted/empty).
/// Runs on a blocking thread so the async runtime / UI never stalls.
#[tauri::command]
async fn scan_mods(
    app: tauri::AppHandle,
    roots: Option<Vec<String>>,
) -> Result<scan::ScanResult, String> {
    let roots: Vec<PathBuf> = match roots {
        Some(v) if !v.is_empty() => v.into_iter().map(PathBuf::from).collect(),
        _ => fsgame::default_mods_paths(),
    };

    tauri::async_runtime::spawn_blocking(move || {
        // Warm cache: parsed entries persist between launches, keyed by
        // path+mtime+size, so unchanged mods skip archive parsing entirely.
        let mut conn = db::open(&db_path(&app)?)?;
        let cache = db::load_cache(&conn);

        let emitter = app.clone();
        let out = scan::scan_cached(roots, &cache, move |done, total| {
            let _ = emitter.emit("scan:progress", Progress { done, total });
        });

        // Persist freshly-parsed rows; prune mods that vanished from disk.
        let fresh_rows: Vec<(String, u64, u64, String)> = out
            .result
            .mods
            .iter()
            .filter(|m| out.fresh_paths.contains(&m.path))
            .filter_map(|m| {
                serde_json::to_string(m)
                    .ok()
                    .map(|json| (m.path.clone(), m.mtime_ms, m.size, json))
            })
            .collect();
        let _ = db::upsert_many(&mut conn, &fresh_rows);

        let present: HashSet<String> = out.result.mods.iter().map(|m| m.path.clone()).collect();
        let _ = db::prune_missing(&mut conn, &present);

        Ok::<_, String>(out.result)
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Return a mod's icon as a cached PNG `data:` URL (or null if unavailable).
/// Decodes off-thread; cached to the app cache dir on first use.
#[tauri::command]
async fn get_mod_icon(
    app: tauri::AppHandle,
    path: String,
    kind: String,
    icon_filename: Option<String>,
) -> Option<String> {
    let icon = icon_filename?;
    let cache_dir = app.path().app_cache_dir().ok()?.join("icons");
    tauri::async_runtime::spawn_blocking(move || {
        icons::cached_data_url(&cache_dir, std::path::Path::new(&path), &kind, &icon)
    })
    .await
    .ok()
    .flatten()
}

// ── Curation (favorite / hidden / broken) ──
#[tauri::command]
fn get_curation(app: tauri::AppHandle) -> Result<Vec<db::CurationRow>, String> {
    let conn = db::open(&db_path(&app)?)?;
    Ok(db::load_curation(&conn))
}

#[tauri::command]
fn set_curation(app: tauri::AppHandle, row: db::CurationRow) -> Result<(), String> {
    let conn = db::open(&db_path(&app)?)?;
    db::set_curation(&conn, &row)
}

// ── Manual category overrides ──
#[tauri::command]
fn get_overrides(app: tauri::AppHandle) -> Result<Vec<db::CategoryOverride>, String> {
    let conn = db::open(&db_path(&app)?)?;
    Ok(db::load_overrides(&conn))
}

#[tauri::command]
fn set_override(app: tauri::AppHandle, row: db::CategoryOverride) -> Result<(), String> {
    let conn = db::open(&db_path(&app)?)?;
    db::set_override(&conn, &row)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            default_mods_paths,
            scan_mods,
            get_mod_icon,
            get_curation,
            set_curation,
            get_overrides,
            set_override
        ])
        .run(tauri::generate_context!())
        .expect("error while running Silo");
}
