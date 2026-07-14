//! FS25 game-file discovery. Cross-platform; see docs/CROSS-PLATFORM.md. This first
//! slice covers the default user mods folder per OS. Steam-library / Epic / GIANTS
//! detection and Proton-prefix handling come next.

use std::path::PathBuf;

/// The FS25 user data directory, per platform.
/// - Windows: `Documents/My Games/FarmingSimulator2025`
/// - macOS:   `~/Library/Application Support/FarmingSimulator2025` (to verify)
/// - Linux:   Proton prefix (not handled yet) — returns None for now.
pub fn user_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        dirs::document_dir().map(|d| d.join("My Games").join("FarmingSimulator2025"))
    }
    #[cfg(target_os = "macos")]
    {
        dirs::data_dir().map(|d| d.join("FarmingSimulator2025"))
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        // Linux/Proton prefix discovery is a TODO (see CROSS-PLATFORM.md).
        None
    }
}

/// Default mod root(s) the game reads. Currently the single `mods/` folder under
/// the user dir; multi-root + `modsDirectoryOverride` come with settings parsing.
pub fn default_mods_paths() -> Vec<PathBuf> {
    match user_dir() {
        Some(dir) => {
            let mods = dir.join("mods");
            if mods.is_dir() {
                vec![mods]
            } else {
                vec![]
            }
        }
        None => vec![],
    }
}
