//! Detect the FS25 install and launch it. Because the active set is already
//! projected into the flat mods folder the game reads, "launch with this loadout"
//! is simply starting FS25 — we do it via Steam so it's tracked properly.

use serde::Serialize;
use std::path::{Path, PathBuf};

/// FS25 on Steam.
const APP_ID: &str = "2300320";
const INSTALL_DIR: &str = "Farming Simulator 25";

#[cfg(windows)]
const EXE: &str = "FarmingSimulator2025.exe";
#[cfg(not(windows))]
const EXE: &str = "FarmingSimulator2025";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameInfo {
    pub app_id: String,
    pub exe: String,
    pub install_dir: String,
}

/// Candidate Steam install roots per platform.
fn steam_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    #[cfg(windows)]
    {
        for p in ["C:/Program Files (x86)/Steam", "C:/Program Files/Steam"] {
            roots.push(PathBuf::from(p));
        }
    }
    #[cfg(target_os = "macos")]
    if let Some(h) = dirs::home_dir() {
        roots.push(h.join("Library/Application Support/Steam"));
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    if let Some(h) = dirs::home_dir() {
        roots.push(h.join(".steam/steam"));
        roots.push(h.join(".local/share/Steam"));
    }
    roots.into_iter().filter(|r| r.join("steamapps").is_dir()).collect()
}

/// Library paths from a Steam root's `libraryfolders.vdf` (plus the root itself).
fn library_paths(steam_root: &Path) -> Vec<PathBuf> {
    let mut paths = vec![steam_root.to_path_buf()];
    let vdf = steam_root.join("steamapps/libraryfolders.vdf");
    if let Ok(content) = std::fs::read_to_string(&vdf) {
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("\"path\"") {
                // `"path"    "E:\\SteamLibrary"` -> 4th quoted field.
                if let Some(val) = line.split('"').nth(3) {
                    paths.push(PathBuf::from(val.replace("\\\\", "/").replace('\\', "/")));
                }
            }
        }
    }
    paths
}

/// Locate the FS25 install across all Steam libraries.
pub fn detect() -> Option<GameInfo> {
    for steam in steam_roots() {
        for lib in library_paths(&steam) {
            let install = lib.join("steamapps/common").join(INSTALL_DIR);
            let exe = install.join(EXE);
            if exe.is_file() {
                return Some(GameInfo {
                    app_id: APP_ID.to_string(),
                    exe: exe.to_string_lossy().into_owned(),
                    install_dir: install.to_string_lossy().into_owned(),
                });
            }
        }
    }
    None
}

/// Launch FS25 through Steam (`steam://rungameid/<appid>`).
pub fn launch() -> Result<(), String> {
    let url = format!("steam://rungameid/{APP_ID}");
    open_url(&url)
}

fn open_url(url: &str) -> Result<(), String> {
    #[cfg(windows)]
    let mut cmd = {
        let mut c = std::process::Command::new("cmd");
        c.args(["/C", "start", "", url]);
        c
    };
    #[cfg(target_os = "macos")]
    let mut cmd = {
        let mut c = std::process::Command::new("open");
        c.arg(url);
        c
    };
    #[cfg(all(unix, not(target_os = "macos")))]
    let mut cmd = {
        let mut c = std::process::Command::new("xdg-open");
        c.arg(url);
        c
    };

    cmd.spawn().map(|_| ()).map_err(|e| e.to_string())
}
