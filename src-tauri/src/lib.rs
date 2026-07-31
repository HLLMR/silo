//! Silo core — Tauri command surface. All heavy logic lives in sibling modules so
//! it stays unit-testable (and reusable by a future CLI) without a running app.

pub mod bindings;
pub mod bisect;
pub mod bridge;
pub mod category;
pub mod conflicts;
pub mod db;
pub mod fsgame;
pub mod gamelaunch;
pub mod github;
pub mod icons;
pub mod logscan;
pub mod moddesc;
pub mod mpsync;
pub mod nexus;
pub mod organize;
pub mod paths;
pub mod savegame;
pub mod scan;
pub mod secrets;
pub mod settings_form;
pub mod siloapi;
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

/// Silo's own GitHub OAuth App client id, baked in so end users get one-click
/// "Connect GitHub" (device flow) with no manual setup. Register ONE OAuth App
/// under the HLLMR account (github.com/settings/applications/new), enable
/// "Device Flow", and paste its Client ID (Iv1.…/Ov23…) here. Client ids are
/// public — safe to embed; device flow needs no secret. Empty = fall back to a
/// user-provided id from Settings.
const SILO_GH_CLIENT_ID: &str = "Ov23lizY2TSJF2P5CRyx";

fn effective_client_id(conn: &rusqlite::Connection) -> Option<String> {
    if !SILO_GH_CLIENT_ID.is_empty() {
        Some(SILO_GH_CLIENT_ID.to_string())
    } else {
        db::get_app_setting(conn, "gh_client_id")
    }
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

/// Scan a mod's modDesc.xml for a github.com/owner/repo reference.
#[tauri::command]
fn guess_repo(path: String, kind: String) -> Option<db::RepoRow> {
    let xml = scan::read_moddesc_xml(std::path::Path::new(&path), &kind).ok()?;
    let (owner, repo) = github::find_repo_in_text(&xml)?;
    Some(db::RepoRow {
        tech_name: String::new(),
        owner,
        repo,
    })
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
        let token = secrets::get(&conn, "gh_token");
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
    /// True when a client id is baked into the app (no user setup needed).
    builtin: bool,
    /// True when the stored token can act (star/watch): a `public_repo` OAuth grant
    /// or a PAT. Read-only (`read:user`) connections report false.
    can_write: bool,
}

#[tauri::command]
fn gh_status(app: tauri::AppHandle) -> Result<GhStatus, String> {
    let conn = db::open(&db_path(&app)?)?;
    Ok(GhStatus {
        client_id: effective_client_id(&conn),
        user: db::get_app_setting(&conn, "gh_user"),
        builtin: !SILO_GH_CLIENT_ID.is_empty(),
        can_write: db::get_app_setting(&conn, "gh_write").as_deref() == Some("1"),
    })
}

#[tauri::command]
fn gh_set_client_id(app: tauri::AppHandle, client_id: String) -> Result<(), String> {
    let conn = db::open(&db_path(&app)?)?;
    let v = client_id.trim();
    db::set_app_setting(
        &conn,
        "gh_client_id",
        if v.is_empty() { None } else { Some(v) },
    )
}

#[tauri::command]
async fn gh_device_start(
    app: tauri::AppHandle,
    write: Option<bool>,
) -> Result<github::DeviceCode, String> {
    let db = db_path(&app)?;
    // Only ask for `public_repo` when the user is enabling actions — a plain sign-in
    // for update-checks stays read-only.
    let scope = if write.unwrap_or(false) {
        "read:user public_repo"
    } else {
        "read:user"
    };
    tauri::async_runtime::spawn_blocking(move || -> Result<github::DeviceCode, String> {
        let conn = db::open(&db)?;
        let cid = effective_client_id(&conn)
            .ok_or_else(|| "No GitHub OAuth App Client ID configured".to_string())?;
        github::device_start(&cid, scope)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
async fn gh_device_poll(
    app: tauri::AppHandle,
    device_code: String,
    write: Option<bool>,
) -> Result<github::PollResult, String> {
    let db = db_path(&app)?;
    let write = write.unwrap_or(false);
    tauri::async_runtime::spawn_blocking(move || -> Result<github::PollResult, String> {
        let conn = db::open(&db)?;
        let cid =
            effective_client_id(&conn).ok_or_else(|| "No client id configured".to_string())?;
        let res = github::device_poll(&cid, &device_code)?;
        if res.status == "ok" {
            if let Some(tok) = &res.token {
                let user = github::whoami(tok).unwrap_or_default();
                secrets::set(&conn, "gh_token", Some(tok))?;
                db::set_app_setting(&conn, "gh_user", Some(&user))?;
                db::set_app_setting(&conn, "gh_write", if write { Some("1") } else { None })?;
            }
        }
        // Never expose the raw token to the frontend.
        Ok(github::PollResult { token: None, ..res })
    })
    .await
    .map_err(|e| e.to_string())?
}

/// PAT fallback for users who'd rather mint a token than OAuth. A fine-grained PAT
/// scoped to only "Starring" (+ optionally "Watching") is the most minimal-permission
/// path — narrower than the OAuth `public_repo` scope. We verify it, then treat it as
/// action-capable.
#[tauri::command]
async fn gh_set_pat(app: tauri::AppHandle, pat: String) -> Result<String, String> {
    let db = db_path(&app)?;
    tauri::async_runtime::spawn_blocking(move || -> Result<String, String> {
        let pat = pat.trim().to_string();
        if pat.is_empty() {
            return Err("Empty token".into());
        }
        let user = github::whoami(&pat)?;
        if user.is_empty() {
            return Err("GitHub did not recognize that token".into());
        }
        let conn = db::open(&db)?;
        secrets::set(&conn, "gh_token", Some(&pat))?;
        db::set_app_setting(&conn, "gh_user", Some(&user))?;
        db::set_app_setting(&conn, "gh_write", Some("1"))?;
        Ok(user)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
async fn gh_repo_stats(
    app: tauri::AppHandle,
    owner: String,
    repo: String,
) -> Result<github::RepoStats, String> {
    let db = db_path(&app)?;
    tauri::async_runtime::spawn_blocking(move || -> Result<github::RepoStats, String> {
        let conn = db::open(&db)?;
        let token = secrets::get(&conn, "gh_token");
        github::repo_stats(&owner, &repo, token.as_deref())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
async fn gh_star(
    app: tauri::AppHandle,
    owner: String,
    repo: String,
    on: bool,
) -> Result<bool, String> {
    let db = db_path(&app)?;
    tauri::async_runtime::spawn_blocking(move || -> Result<bool, String> {
        let conn = db::open(&db)?;
        let token = secrets::get(&conn, "gh_token")
            .ok_or_else(|| "Connect your GitHub account first".to_string())?;
        github::set_star(&owner, &repo, &token, on)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
async fn gh_watch(
    app: tauri::AppHandle,
    owner: String,
    repo: String,
    on: bool,
) -> Result<bool, String> {
    let db = db_path(&app)?;
    tauri::async_runtime::spawn_blocking(move || -> Result<bool, String> {
        let conn = db::open(&db)?;
        let token = secrets::get(&conn, "gh_token")
            .ok_or_else(|| "Connect your GitHub account first".to_string())?;
        github::set_watch(&owner, &repo, &token, on)
    })
    .await
    .map_err(|e| e.to_string())?
}

// ── Nexus source card (keyless reads + endorse via the user's own API key) ──

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct NexusStatus {
    user: Option<String>,
}

#[tauri::command]
fn nexus_status(app: tauri::AppHandle) -> Result<NexusStatus, String> {
    let conn = db::open(&db_path(&app)?)?;
    Ok(NexusStatus {
        user: db::get_app_setting(&conn, "nexus_user"),
    })
}

/// Verify + store a Nexus personal API key. Returns the account name.
#[tauri::command]
async fn nexus_set_key(app: tauri::AppHandle, key: String) -> Result<String, String> {
    let db = db_path(&app)?;
    tauri::async_runtime::spawn_blocking(move || -> Result<String, String> {
        let key = key.trim().to_string();
        if key.is_empty() {
            return Err("Empty key".into());
        }
        let user = nexus::validate_key(&key)?;
        let conn = db::open(&db)?;
        secrets::set(&conn, "nexus_key", Some(&key))?;
        db::set_app_setting(&conn, "nexus_user", Some(&user))?;
        Ok(user)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
fn nexus_logout(app: tauri::AppHandle) -> Result<(), String> {
    let conn = db::open(&db_path(&app)?)?;
    secrets::set(&conn, "nexus_key", None)?;
    db::set_app_setting(&conn, "nexus_user", None)?;
    Ok(())
}

#[tauri::command]
async fn nexus_mod(app: tauri::AppHandle, mod_id: u64) -> Result<nexus::NexusMod, String> {
    let db = db_path(&app)?;
    tauri::async_runtime::spawn_blocking(move || -> Result<nexus::NexusMod, String> {
        let conn = db::open(&db)?;
        let key = secrets::get(&conn, "nexus_key");
        nexus::mod_stats(mod_id, key.as_deref())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
async fn nexus_endorse(
    app: tauri::AppHandle,
    mod_id: u64,
    on: bool,
    version: Option<String>,
) -> Result<bool, String> {
    let db = db_path(&app)?;
    tauri::async_runtime::spawn_blocking(move || -> Result<bool, String> {
        let conn = db::open(&db)?;
        let key = secrets::get(&conn, "nexus_key")
            .ok_or_else(|| "Add your Nexus API key first".to_string())?;
        nexus::set_endorse(mod_id, &key, on, version.as_deref())
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Full Nexus mod body (keyless), cleaned to readable text — for the description modal.
#[tauri::command]
async fn nexus_description(mod_id: u64) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || nexus::mod_description(mod_id))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
async fn download_update(
    app: tauri::AppHandle,
    path: String,
    asset_url: String,
) -> Result<(), String> {
    let db = db_path(&app)?;
    tauri::async_runtime::spawn_blocking(move || -> Result<(), String> {
        let dest = std::path::Path::new(&path);
        paths::no_traversal(dest)?;
        let conn = db::open(&db)?;
        let token = secrets::get(&conn, "gh_token");
        github::download_zip(&asset_url, token.as_deref(), dest)
    })
    .await
    .map_err(|e| e.to_string())?
}

// ── SiloAPI (mod browser / discovery) ──

fn siloapi_base(app: &tauri::AppHandle) -> Result<String, String> {
    let conn = db::open(&db_path(app)?)?;
    Ok(db::get_app_setting(&conn, "siloapi_base")
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| siloapi::DEFAULT_BASE.to_string()))
}

#[tauri::command]
fn siloapi_status(app: tauri::AppHandle) -> Result<String, String> {
    siloapi_base(&app)
}

#[tauri::command]
fn siloapi_set_base(app: tauri::AppHandle, base: String) -> Result<(), String> {
    let conn = db::open(&db_path(&app)?)?;
    let v = base.trim().trim_end_matches('/');
    db::set_app_setting(
        &conn,
        "siloapi_base",
        if v.is_empty() { None } else { Some(v) },
    )
}

#[tauri::command]
async fn browse_mods(
    app: tauri::AppHandle,
    query: Option<String>,
    category: Option<String>,
    sort: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<siloapi::BrowsePage, String> {
    let base = siloapi_base(&app)?;
    tauri::async_runtime::spawn_blocking(move || {
        siloapi::browse(
            &base,
            query.as_deref(),
            category.as_deref(),
            sort.as_deref(),
            limit.unwrap_or(40),
            offset.unwrap_or(0),
        )
    })
    .await
    .map_err(|e| e.to_string())?
}

/// One mod's full catalog record (metadata + every source it was seen on).
#[tauri::command]
async fn siloapi_mod_detail(
    app: tauri::AppHandle,
    id: String,
) -> Result<siloapi::BrowseMod, String> {
    let base = siloapi_base(&app)?;
    tauri::async_runtime::spawn_blocking(move || siloapi::detail(&base, &id))
        .await
        .map_err(|e| e.to_string())?
}

/// Catalog categories with counts, for the Browse filter.
#[tauri::command]
async fn siloapi_categories(app: tauri::AppHandle) -> Result<Vec<siloapi::CategoryCount>, String> {
    let base = siloapi_base(&app)?;
    tauri::async_runtime::spawn_blocking(move || siloapi::categories(&base))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
async fn siloapi_stats(app: tauri::AppHandle) -> Result<siloapi::Stats, String> {
    let base = siloapi_base(&app)?;
    tauri::async_runtime::spawn_blocking(move || siloapi::stats(&base))
        .await
        .map_err(|e| e.to_string())?
}

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct InstallProgress {
    id: String,
    done: u64,
    total: Option<u64>,
}

/// Download a browsed mod's .zip into the library root and return the filename.
/// Streams the download, emitting `install:progress` events per mod id so the UI can
/// show a bar. The frontend rescans (and auto-files) afterwards.
#[tauri::command]
async fn install_remote_mod(
    app: tauri::AppHandle,
    id: String,
    source: Option<String>,
    root: Option<String>,
) -> Result<String, String> {
    let base = siloapi_base(&app)?;
    let root = primary_root(root)?;
    let emitter = app.clone();
    tauri::async_runtime::spawn_blocking(move || -> Result<String, String> {
        let (url, filename) = siloapi::resolve_download(&base, &id, source.as_deref())?;
        // The filename comes from the catalog — validate it's a plain basename so a
        // hostile response can't redirect the write outside the mods folder.
        paths::safe_file_name(&filename)?;
        let dest = root.join(&filename);
        if dest.exists() {
            return Err(format!("{filename} is already in your library"));
        }
        siloapi::download_to(&url, &dest, |done, total| {
            let _ = emitter.emit(
                "install:progress",
                InstallProgress {
                    id: id.clone(),
                    done,
                    total,
                },
            );
        })?;
        Ok(filename)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct LocalMod {
    tech_name: String,
    version: Option<String>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct CatalogUpdate {
    tech_name: String,
    latest: Option<String>,
    has_update: bool,
    download_url: Option<String>,
    source: Option<String>,
}

/// Check the whole library against the SiloAPI catalog in one request (by tech name),
/// returning which mods have a newer version and where to get it. Covers ModHub mods
/// too — not just GitHub-linked ones. Only mods the catalog knows are returned.
#[tauri::command]
async fn catalog_check_updates(
    app: tauri::AppHandle,
    mods: Vec<LocalMod>,
) -> Result<Vec<CatalogUpdate>, String> {
    let base = siloapi_base(&app)?;
    tauri::async_runtime::spawn_blocking(move || -> Result<Vec<CatalogUpdate>, String> {
        let names: Vec<String> = mods.iter().map(|m| m.tech_name.clone()).collect();
        let results = siloapi::lookup(&base, &names)?;
        let by_tech: std::collections::HashMap<&str, &siloapi::LookupResult> = results
            .iter()
            .filter_map(|r| r.tech_name.as_deref().map(|t| (t, r)))
            .collect();

        let mut out = Vec::new();
        for m in &mods {
            let Some(r) = by_tech.get(m.tech_name.as_str()) else {
                continue;
            };
            let current = m.version.clone().unwrap_or_default();
            let has_update = r
                .latest_version
                .as_deref()
                .is_some_and(|latest| github::is_newer(latest, &current));
            out.push(CatalogUpdate {
                tech_name: m.tech_name.clone(),
                latest: r.latest_version.clone(),
                has_update,
                download_url: r.download.as_ref().map(|d| d.url.clone()),
                source: r.download.as_ref().map(|d| d.source.clone()),
            });
        }
        Ok(out)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
fn gh_logout(app: tauri::AppHandle) -> Result<(), String> {
    let conn = db::open(&db_path(&app)?)?;
    secrets::set(&conn, "gh_token", None)?;
    db::set_app_setting(&conn, "gh_user", None)?;
    db::set_app_setting(&conn, "gh_write", None)?;
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
    let file = LoadoutFile {
        silo: 1,
        name: lo.name,
        mods: lo.mods,
    };
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

/// Clear the scan cache so the next scan re-parses & re-categorizes every mod.
#[tauri::command]
fn clear_scan_cache(app: tauri::AppHandle) -> Result<(), String> {
    let conn = db::open(&db_path(&app)?)?;
    db::clear_cache(&conn);
    Ok(())
}

/// The FS25 user data dir (parent of mods/, savegameN/, game.xml).
#[tauri::command]
fn user_dir_path() -> Option<String> {
    fsgame::user_dir().map(|p| p.to_string_lossy().into_owned())
}

/// Generate a filltype-compatibility bridge companion mod into the library root.
/// Returns the created zip filename; the frontend rescans to pick it up.
#[tauri::command]
async fn generate_bridge(spec: bridge::BridgeSpec, root: Option<String>) -> Result<String, String> {
    let root = primary_root(root)?;
    tauri::async_runtime::spawn_blocking(move || -> Result<String, String> {
        let filename = format!("{}.zip", spec.tech_name);
        let dest = root.join(&filename);
        if dest.exists() {
            return Err(format!("{filename} already exists in your mods folder"));
        }
        bridge::generate(&spec, &dest)?;
        Ok(filename)
    })
    .await
    .map_err(|e| e.to_string())?
}

// ── Multiplayer mod-set sync ──

/// Hash the host's active set and write a shareable manifest to `path`. Returns the
/// number of mods written.
#[tauri::command]
async fn mp_export(mods: Vec<mpsync::ModRef>, path: String) -> Result<usize, String> {
    tauri::async_runtime::spawn_blocking(move || -> Result<usize, String> {
        let manifest = mpsync::build_manifest(&mods);
        let json = serde_json::to_string_pretty(&manifest).map_err(|e| e.to_string())?;
        std::fs::write(&path, json).map_err(|e| e.to_string())?;
        Ok(manifest.mods.len())
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Read a shared manifest from `path`, hash the joiner's local set, and diff.
#[tauri::command]
async fn mp_verify_file(
    path: String,
    local: Vec<mpsync::ModRef>,
) -> Result<mpsync::VerifyReport, String> {
    tauri::async_runtime::spawn_blocking(move || -> Result<mpsync::VerifyReport, String> {
        let text = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let manifest: mpsync::Manifest = serde_json::from_str(&text)
            .map_err(|_| "That file isn't a valid Silo mod-set manifest".to_string())?;
        let local_hashed = mpsync::hash_mods(&local);
        Ok(mpsync::diff(&manifest.mods, &local_hashed))
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Parse the FS25 log.txt and report which mods are throwing errors/warnings, whether
/// the last run crashed, and the culprit ranking. Read-only; touches nothing.
#[tauri::command]
async fn scan_log() -> Result<logscan::LogReport, String> {
    let path = fsgame::user_dir()
        .map(|d| d.join("log.txt"))
        .ok_or_else(|| "Could not locate the FS25 user directory".to_string())?;
    tauri::async_runtime::spawn_blocking(move || -> Result<logscan::LogReport, String> {
        if !path.exists() {
            return Err("No log.txt found — launch FS25 at least once first".to_string());
        }
        // Logs are read as lossy UTF-8; FS writes mostly ASCII but paths can carry stray bytes.
        let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
        let text = String::from_utf8_lossy(&bytes);
        Ok(logscan::parse(&text))
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Parse inputBinding.xml into a per-device binding map (read-only).
#[tauri::command]
async fn scan_bindings() -> Result<bindings::BindingReport, String> {
    let path = fsgame::user_dir()
        .map(|d| d.join("inputBinding.xml"))
        .ok_or_else(|| "Could not locate the FS25 user directory".to_string())?;
    tauri::async_runtime::spawn_blocking(move || -> Result<bindings::BindingReport, String> {
        if !path.exists() {
            return Err("No inputBinding.xml found — launch FS25 at least once first".to_string());
        }
        let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
        Ok(bindings::parse(&String::from_utf8_lossy(&bytes)))
    })
    .await
    .map_err(|e| e.to_string())?
}

// ── Guided bisection ──

/// Decide the next split for the current suspect pool (pure; see bisect.rs).
#[tauri::command]
fn bisect_plan(pool: Vec<String>) -> bisect::BisectStep {
    bisect::plan(&pool)
}

/// Narrow the pool by the last round's verdict — kept in one tested place so the
/// test/rest halves can't get swapped by the caller.
#[tauri::command]
fn bisect_narrow(test: Vec<String>, rest: Vec<String>, still_broken: bool) -> Vec<String> {
    bisect::narrow(test, rest, still_broken)
}

/// Persist the user's real active set before bisection perturbs it, so even an app
/// crash mid-diagnosis can restore it (reversibility is non-negotiable — CLAUDE.md).
#[tauri::command]
fn bisect_snapshot_save(app: tauri::AppHandle, active: Vec<String>) -> Result<(), String> {
    let conn = db::open(&db_path(&app)?)?;
    let json = serde_json::to_string(&active).map_err(|e| e.to_string())?;
    db::set_app_setting(&conn, "bisect_snapshot", Some(&json))
}

/// The saved pre-bisection active set, if a bisection is (or was) in progress.
#[tauri::command]
fn bisect_snapshot_get(app: tauri::AppHandle) -> Result<Option<Vec<String>>, String> {
    let conn = db::open(&db_path(&app)?)?;
    match db::get_app_setting(&conn, "bisect_snapshot") {
        Some(json) => Ok(Some(
            serde_json::from_str(&json).map_err(|e| e.to_string())?,
        )),
        None => Ok(None),
    }
}

/// Clear the snapshot once the original set has been restored.
#[tauri::command]
fn bisect_snapshot_clear(app: tauri::AppHandle) -> Result<(), String> {
    let conn = db::open(&db_path(&app)?)?;
    db::set_app_setting(&conn, "bisect_snapshot", None)
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
            guess_repo,
            check_mod_update,
            gh_status,
            gh_set_client_id,
            gh_device_start,
            gh_device_poll,
            gh_set_pat,
            gh_repo_stats,
            gh_star,
            gh_watch,
            gh_logout,
            nexus_status,
            nexus_set_key,
            nexus_logout,
            nexus_mod,
            nexus_endorse,
            nexus_description,
            download_update,
            siloapi_status,
            siloapi_set_base,
            browse_mods,
            siloapi_stats,
            siloapi_categories,
            siloapi_mod_detail,
            install_remote_mod,
            catalog_check_updates,
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
            clear_scan_cache,
            scan_log,
            scan_bindings,
            mp_export,
            mp_verify_file,
            generate_bridge,
            bisect_plan,
            bisect_narrow,
            bisect_snapshot_save,
            bisect_snapshot_get,
            bisect_snapshot_clear,
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
