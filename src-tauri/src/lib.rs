//! Silo core — Tauri command surface. All heavy logic lives in sibling modules so
//! it stays unit-testable (and reusable by a future CLI) without a running app.

pub mod category;
pub mod conflicts;
pub mod db;
pub mod fsgame;
pub mod gamelaunch;
pub mod github;
pub mod icons;
pub mod moddesc;
pub mod organize;
pub mod savegame;
pub mod scan;
pub mod settings_form;
pub mod store;
pub mod xmlconfig;

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

// ── GitHub update checking ──
#[tauri::command]
fn get_mod_repos(app: tauri::AppHandle) -> Result<Vec<db::RepoRow>, String> {
    let conn = db::open(&db_path(&app)?)?;
    Ok(db::load_repos(&conn))
}

#[tauri::command]
fn set_mod_repo(
    app: tauri::AppHandle,
    tech_name: String,
    owner: String,
    repo: String,
) -> Result<(), String> {
    let conn = db::open(&db_path(&app)?)?;
    db::set_repo(&conn, &tech_name, &owner, &repo)
}

#[tauri::command]
async fn check_mod_update(
    app: tauri::AppHandle,
    owner: String,
    repo: String,
    current: String,
) -> Result<github::UpdateInfo, String> {
    let db = db_path(&app)?;
    tauri::async_runtime::spawn_blocking(move || -> Result<github::UpdateInfo, String> {
        let conn = db::open(&db)?;
        let token = db::get_app_setting(&conn, "gh_token");
        github::check(&owner, &repo, &current, token.as_deref())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct GhStatus {
    client_id: Option<String>,
    user: Option<String>,
}

#[tauri::command]
fn gh_status(app: tauri::AppHandle) -> Result<GhStatus, String> {
    let conn = db::open(&db_path(&app)?)?;
    Ok(GhStatus {
        client_id: db::get_app_setting(&conn, "gh_client_id"),
        user: db::get_app_setting(&conn, "gh_user"),
    })
}

#[tauri::command]
fn gh_set_client_id(app: tauri::AppHandle, client_id: String) -> Result<(), String> {
    let conn = db::open(&db_path(&app)?)?;
    let v = client_id.trim();
    db::set_app_setting(&conn, "gh_client_id", if v.is_empty() { None } else { Some(v) })
}

#[tauri::command]
async fn gh_device_start(app: tauri::AppHandle) -> Result<github::DeviceCode, String> {
    let db = db_path(&app)?;
    tauri::async_runtime::spawn_blocking(move || -> Result<github::DeviceCode, String> {
        let conn = db::open(&db)?;
        let cid = db::get_app_setting(&conn, "gh_client_id")
            .ok_or_else(|| "Set a GitHub OAuth App Client ID in Settings first".to_string())?;
        github::device_start(&cid)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
async fn gh_device_poll(
    app: tauri::AppHandle,
    device_code: String,
) -> Result<github::PollResult, String> {
    let db = db_path(&app)?;
    tauri::async_runtime::spawn_blocking(move || -> Result<github::PollResult, String> {
        let conn = db::open(&db)?;
        let cid = db::get_app_setting(&conn, "gh_client_id")
            .ok_or_else(|| "No client id configured".to_string())?;
        let res = github::device_poll(&cid, &device_code)?;
        if res.status == "ok" {
            if let Some(tok) = &res.token {
                let user = github::whoami(tok).unwrap_or_default();
                db::set_app_setting(&conn, "gh_token", Some(tok))?;
                db::set_app_setting(&conn, "gh_user", Some(&user))?;
            }
        }
        // Never expose the raw token to the frontend.
        Ok(github::PollResult { token: None, ..res })
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
fn gh_logout(app: tauri::AppHandle) -> Result<(), String> {
    let conn = db::open(&db_path(&app)?)?;
    db::set_app_setting(&conn, "gh_token", None)?;
    db::set_app_setting(&conn, "gh_user", None)?;
    Ok(())
}

// ── Tags ──
#[tauri::command]
fn get_tags(app: tauri::AppHandle) -> Result<Vec<db::TagRow>, String> {
    let conn = db::open(&db_path(&app)?)?;
    Ok(db::load_tags(&conn))
}

#[tauri::command]
fn set_tags(app: tauri::AppHandle, tech_name: String, tags: Vec<String>) -> Result<(), String> {
    let mut conn = db::open(&db_path(&app)?)?;
    db::set_tags(&mut conn, &tech_name, &tags)
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

// ── Organize / projection engine (writes to the game folder) ──
fn primary_root(root: Option<String>) -> Result<PathBuf, String> {
    match root {
        Some(r) if !r.is_empty() => Ok(PathBuf::from(r)),
        _ => fsgame::default_mods_paths()
            .into_iter()
            .next()
            .ok_or_else(|| "No mods folder detected".to_string()),
    }
}

/// Dry run: what organizing would move (read-only).
#[tauri::command]
async fn plan_organize(
    root: Option<String>,
    mods: Vec<organize::ModInput>,
) -> Result<Vec<organize::PlannedMove>, String> {
    let root = primary_root(root)?;
    tauri::async_runtime::spawn_blocking(move || organize::plan_organize(&root, &mods))
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn apply_organize(
    app: tauri::AppHandle,
    root: Option<String>,
    mods: Vec<organize::ModInput>,
) -> Result<organize::Report, String> {
    let db = db_path(&app)?;
    let root = primary_root(root)?;
    tauri::async_runtime::spawn_blocking(move || -> Result<organize::Report, String> {
        let conn = db::open(&db)?;
        Ok(organize::apply_organize(&conn, &root, &mods))
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
async fn set_active(
    app: tauri::AppHandle,
    root: Option<String>,
    active: Vec<String>,
) -> Result<organize::Report, String> {
    let db = db_path(&app)?;
    let root = primary_root(root)?;
    tauri::async_runtime::spawn_blocking(move || -> Result<organize::Report, String> {
        let conn = db::open(&db)?;
        let set: HashSet<String> = active.into_iter().collect();
        Ok(organize::set_active(&conn, &root, &set))
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
async fn flatten(app: tauri::AppHandle, root: Option<String>) -> Result<organize::Report, String> {
    let db = db_path(&app)?;
    let root = primary_root(root)?;
    tauri::async_runtime::spawn_blocking(move || -> Result<organize::Report, String> {
        let conn = db::open(&db)?;
        Ok(organize::flatten(&conn, &root))
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
fn get_organized(app: tauri::AppHandle) -> Result<Vec<db::OrganizedRow>, String> {
    let conn = db::open(&db_path(&app)?)?;
    Ok(db::load_organized(&conn))
}

// ── Loadouts (named active-mod sets) ──
#[tauri::command]
fn get_loadouts(app: tauri::AppHandle) -> Result<Vec<db::Loadout>, String> {
    let conn = db::open(&db_path(&app)?)?;
    Ok(db::load_loadouts(&conn))
}

#[tauri::command]
fn save_loadout(
    app: tauri::AppHandle,
    id: Option<i64>,
    name: String,
    mods: Vec<String>,
) -> Result<i64, String> {
    let mut conn = db::open(&db_path(&app)?)?;
    db::save_loadout(&mut conn, id, &name, &mods)
}

#[tauri::command]
fn delete_loadout(app: tauri::AppHandle, id: i64) -> Result<(), String> {
    let mut conn = db::open(&db_path(&app)?)?;
    db::delete_loadout(&mut conn, id)
}

#[derive(serde::Serialize, serde::Deserialize)]
struct LoadoutFile {
    silo: u32,
    name: String,
    mods: Vec<String>,
}

#[tauri::command]
fn export_loadout(app: tauri::AppHandle, id: i64, path: String) -> Result<(), String> {
    let conn = db::open(&db_path(&app)?)?;
    let lo = db::load_loadouts(&conn)
        .into_iter()
        .find(|l| l.id == id)
        .ok_or_else(|| "Loadout not found".to_string())?;
    let file = LoadoutFile { silo: 1, name: lo.name, mods: lo.mods };
    let json = serde_json::to_string_pretty(&file).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())
}

#[tauri::command]
fn import_loadout(app: tauri::AppHandle, path: String) -> Result<i64, String> {
    let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let file: LoadoutFile = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    let mut conn = db::open(&db_path(&app)?)?;
    db::save_loadout(&mut conn, None, &file.name, &file.mods)
}

// ── Conflict detection (over the active set) ──
#[tauri::command]
async fn detect_conflicts(
    mods: Vec<conflicts::ConflictInput>,
) -> Result<Vec<conflicts::Conflict>, String> {
    tauri::async_runtime::spawn_blocking(move || conflicts::detect(&mods))
        .await
        .map_err(|e| e.to_string())
}

// ── Game launch ──
#[tauri::command]
fn detect_game() -> Option<gamelaunch::GameInfo> {
    gamelaunch::detect()
}

#[tauri::command]
fn launch_game() -> Result<(), String> {
    gamelaunch::launch()
}

/// Write text to a user-chosen path (used by the diagnostics report export).
#[tauri::command]
fn save_text(path: String, content: String) -> Result<(), String> {
    std::fs::write(&path, content).map_err(|e| e.to_string())
}

/// The FS25 user data dir (parent of mods/, savegameN/, game.xml).
#[tauri::command]
fn user_dir_path() -> Option<String> {
    fsgame::user_dir().map(|p| p.to_string_lossy().into_owned())
}

/// Read specific values from a config XML by path.
#[tauri::command]
fn get_config(
    path: String,
    paths: Vec<String>,
) -> Result<std::collections::HashMap<String, String>, String> {
    let xml = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    Ok(xmlconfig::get_values(&xml, &paths))
}

/// Apply value edits to a config XML, backing up the original to `<file>.bak`.
#[tauri::command]
fn set_config(path: String, edits: Vec<xmlconfig::Edit>) -> Result<(), String> {
    let xml = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let out = xmlconfig::set_values(&xml, &edits)?;
    let _ = std::fs::copy(&path, format!("{path}.bak"));
    std::fs::write(&path, out).map_err(|e| e.to_string())
}

// ── Mod settings form ──
#[tauri::command]
fn mods_with_settings() -> Vec<String> {
    match fsgame::user_dir() {
        Some(dir) => settings_form::mods_with_settings(&dir),
        None => Vec::new(),
    }
}

#[tauri::command]
fn get_mod_settings(mod_name: String) -> Result<Vec<settings_form::SettingsFile>, String> {
    let dir = fsgame::user_dir().ok_or_else(|| "No FS25 user dir".to_string())?;
    let mut files = Vec::new();
    for path in settings_form::find_files(&dir, &mod_name) {
        files.push(settings_form::load_file(&path)?);
    }
    Ok(files)
}

#[tauri::command]
fn save_mod_settings(path: String, edits: Vec<settings_form::Edit>) -> Result<(), String> {
    settings_form::save(std::path::Path::new(&path), &edits)
}

#[tauri::command]
fn save_mod_settings_raw(path: String, content: String) -> Result<(), String> {
    settings_form::save_raw(std::path::Path::new(&path), &content)
}

// ── Savegames ──
#[tauri::command]
fn get_savegames() -> Result<Vec<savegame::Savegame>, String> {
    match fsgame::user_dir() {
        Some(dir) => Ok(savegame::list_savegames(&dir)),
        None => Ok(Vec::new()),
    }
}

#[tauri::command]
fn backup_savegame(folder: String) -> Result<String, String> {
    let dir = fsgame::user_dir().ok_or_else(|| "No FS25 user dir".to_string())?;
    savegame::backup(&dir, &folder)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            default_mods_paths,
            scan_mods,
            get_mod_icon,
            get_curation,
            set_curation,
            get_tags,
            set_tags,
            get_mod_repos,
            set_mod_repo,
            check_mod_update,
            gh_status,
            gh_set_client_id,
            gh_device_start,
            gh_device_poll,
            gh_logout,
            get_overrides,
            set_override,
            plan_organize,
            apply_organize,
            set_active,
            flatten,
            get_organized,
            get_loadouts,
            save_loadout,
            delete_loadout,
            export_loadout,
            import_loadout,
            get_savegames,
            backup_savegame,
            detect_conflicts,
            detect_game,
            launch_game,
            save_text,
            user_dir_path,
            get_config,
            set_config,
            mods_with_settings,
            get_mod_settings,
            save_mod_settings,
            save_mod_settings_raw
        ])
        .run(tauri::generate_context!())
        .expect("error while running Silo");
}
