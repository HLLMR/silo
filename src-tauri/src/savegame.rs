//! Read FS25 savegames and their bound mod lists from `careerSavegame.xml`.
//! Each `<mod modName version required fileHash/>` records what a save was built
//! on; `required="true"` mods are ones the save genuinely needs. Silo uses this to
//! build a loadout from a save and to verify a save's mods are present.

use quick_xml::events::Event;
use quick_xml::reader::Reader;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveMod {
    pub mod_name: String,
    pub title: Option<String>,
    pub version: Option<String>,
    pub required: bool,
    pub file_hash: Option<String>,
    /// pdlc_* entries are paid DLC, not user mods Silo manages.
    pub is_dlc: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Savegame {
    pub index: u32,
    pub folder: String,
    pub name: String,
    pub map_title: Option<String>,
    pub mods: Vec<SaveMod>,
}

fn attr(e: &quick_xml::events::BytesStart, key: &[u8]) -> Option<String> {
    e.attributes()
        .flatten()
        .find(|a| a.key.as_ref() == key)
        .and_then(|a| a.unescape_value().ok().map(|c| c.into_owned()))
}

/// Parse a `careerSavegame.xml` into name/map/mod-list.
fn parse(xml: &str, index: u32, folder: &str) -> Savegame {
    let mut name = String::new();
    let mut map_title = None;
    let mut mods = Vec::new();

    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    let mut stack: Vec<String> = Vec::new();

    let handle_open = |e: &quick_xml::events::BytesStart, mods: &mut Vec<SaveMod>| {
        let local = String::from_utf8_lossy(e.local_name().as_ref()).into_owned();
        if local == "mod" {
            if let Some(mod_name) = attr(e, b"modName") {
                let is_dlc = mod_name.starts_with("pdlc_");
                mods.push(SaveMod {
                    is_dlc,
                    title: attr(e, b"title"),
                    version: attr(e, b"version"),
                    required: attr(e, b"required").as_deref() == Some("true"),
                    file_hash: attr(e, b"fileHash"),
                    mod_name,
                });
            }
        }
        local
    };

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Eof) | Err(_) => break,
            Ok(Event::Start(e)) => {
                let local = handle_open(&e, &mut mods);
                stack.push(local); // real elements have a matching End
            }
            Ok(Event::Empty(e)) => {
                // Self-closing (e.g. <mod .../>): handle attrs, don't push.
                handle_open(&e, &mut mods);
            }
            Ok(Event::End(_)) => {
                stack.pop();
            }
            Ok(Event::Text(t)) => {
                let last = stack.last().map(String::as_str).unwrap_or("");
                let parent = stack.iter().rev().nth(1).map(String::as_str).unwrap_or("");
                if let Ok(txt) = t.unescape() {
                    let txt = txt.trim();
                    if parent == "settings" && last == "savegameName" {
                        name = txt.to_string();
                    } else if parent == "settings" && last == "mapTitle" {
                        map_title = Some(txt.to_string());
                    }
                }
            }
            _ => {}
        }
        buf.clear();
    }

    if name.is_empty() {
        name = folder.to_string();
    }
    Savegame {
        index,
        folder: folder.to_string(),
        name,
        map_title,
        mods,
    }
}

/// List all savegames under the FS25 user dir, parsed and sorted by slot.
pub fn list_savegames(user_dir: &Path) -> Vec<Savegame> {
    let mut out = Vec::new();
    let Ok(rd) = std::fs::read_dir(user_dir) else {
        return out;
    };
    for entry in rd.flatten() {
        let folder = entry.file_name().to_string_lossy().into_owned();
        let Some(index) = folder.strip_prefix("savegame").and_then(|n| n.parse::<u32>().ok()) else {
            continue;
        };
        let xml_path = entry.path().join("careerSavegame.xml");
        let Ok(xml) = std::fs::read_to_string(&xml_path) else {
            continue;
        };
        out.push(parse(&xml, index, &folder));
    }
    out.sort_by_key(|s| s.index);
    out
}

fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for e in std::fs::read_dir(src)? {
        let e = e?;
        let to = dst.join(e.file_name());
        if e.file_type()?.is_dir() {
            copy_dir(&e.path(), &to)?;
        } else {
            std::fs::copy(e.path(), &to)?;
        }
    }
    Ok(())
}

/// Back up a savegame folder to `SiloBackups/<folder>_<epoch>` (a plain copy —
/// non-destructive; the original is untouched). Returns the backup path.
pub fn backup(user_dir: &Path, folder: &str) -> Result<String, String> {
    let src = user_dir.join(folder);
    if !src.is_dir() {
        return Err(format!("savegame '{folder}' not found"));
    }
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let dst = user_dir.join("SiloBackups").join(format!("{folder}_{secs}"));
    copy_dir(&src, &dst).map_err(|e| e.to_string())?;
    Ok(dst.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"<careerSavegame>
        <settings>
            <savegameName>My game save</savegameName>
            <mapTitle>Riverbend Springs</mapTitle>
        </settings>
        <mod modName="pdlc_vredoPack" title="Vredo Pack" version="1.1.0.0" required="false" fileHash="abc"/>
        <mod modName="FS25_precisionFarming" title="Precision Farming" version="1.5.0.0" required="true" fileHash="def"/>
    </careerSavegame>"#;

    #[test]
    fn parses_name_and_mods() {
        let s = parse(SAMPLE, 1, "savegame1");
        assert_eq!(s.name, "My game save");
        assert_eq!(s.map_title.as_deref(), Some("Riverbend Springs"));
        assert_eq!(s.mods.len(), 2);
        let pf = s.mods.iter().find(|m| m.mod_name == "FS25_precisionFarming").unwrap();
        assert!(pf.required && !pf.is_dlc);
        let dlc = s.mods.iter().find(|m| m.mod_name == "pdlc_vredoPack").unwrap();
        assert!(dlc.is_dlc && !dlc.required);
    }

    #[test]
    fn backup_copies_savegame() {
        let root = std::env::temp_dir().join("silo_sg_backup_test");
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("savegame1")).unwrap();
        std::fs::write(root.join("savegame1/careerSavegame.xml"), "<x/>").unwrap();
        let dst = backup(&root, "savegame1").unwrap();
        assert!(Path::new(&dst).join("careerSavegame.xml").is_file());
        // Original untouched.
        assert!(root.join("savegame1/careerSavegame.xml").is_file());
        let _ = std::fs::remove_dir_all(&root);
    }
}
