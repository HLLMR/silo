//! Turn a mod's settings XML into an editable form and write edits back safely.
//!
//! Handles the FS-idiomatic settings shape — elements carrying a typed value in a
//! `boolean`/`integer`/`float`/`string` (or `value`) attribute, e.g.
//! `<setting name="FOO" boolean="true"/>`. Each such attribute becomes a form
//! field; the element's `name` attribute (or tag) is the label. Anything the form
//! can't model stays editable via the raw-XML escape hatch in the UI.
//!
//! Field ids are assigned in a single deterministic document-order walk, and the
//! exact same walk drives write-back — so a field always maps to the same value,
//! and only edited values change (the rest of the file is re-emitted as read).

use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub id: usize,
    pub label: String,
    pub kind: String, // "bool" | "int" | "float" | "string"
    pub value: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsFile {
    pub path: String,
    pub name: String,
    pub fields: Vec<Field>,
    pub raw: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edit {
    pub id: usize,
    pub value: String,
}

struct Slot {
    attr: String,
    kind: String,
    value: String,
    label: String,
}

fn infer_kind(v: &str) -> &'static str {
    let t = v.trim();
    if t == "true" || t == "false" {
        "bool"
    } else if t.parse::<i64>().is_ok() {
        "int"
    } else if t.parse::<f64>().is_ok() {
        "float"
    } else {
        "string"
    }
}

fn local(name: &[u8]) -> String {
    let s = String::from_utf8_lossy(name);
    s.rsplit(':').next().unwrap_or(&s).to_string()
}

/// The editable attribute "slots" of one element, in attribute order.
fn element_slots(e: &BytesStart) -> Vec<Slot> {
    let mut name_label: Option<String> = None;
    let mut raw: Vec<(String, String)> = Vec::new();
    for a in e.attributes().flatten() {
        let key = local(a.key.as_ref());
        let val = a.unescape_value().map(|c| c.into_owned()).unwrap_or_default();
        if key == "name" {
            name_label = Some(val.clone());
        }
        raw.push((key, val));
    }
    let base = name_label.unwrap_or_else(|| local(e.name().as_ref()));

    let mut slots: Vec<Slot> = Vec::new();
    for (key, val) in &raw {
        let kind = match key.as_str() {
            "boolean" => Some("bool"),
            "integer" => Some("int"),
            "float" => Some("float"),
            "string" => Some("string"),
            "value" => Some(infer_kind(val)),
            _ => None,
        };
        if let Some(kind) = kind {
            slots.push(Slot {
                attr: key.clone(),
                kind: kind.to_string(),
                value: val.clone(),
                label: base.clone(),
            });
        }
    }
    // Disambiguate labels when an element holds more than one field.
    if slots.len() > 1 {
        for s in &mut slots {
            s.label = format!("{} · {}", s.label, s.attr);
        }
    }
    slots
}

/// Parse settings XML into form fields.
pub fn parse(xml: &str) -> Vec<Field> {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    let mut fields = Vec::new();
    let mut id = 0usize;
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Eof) | Err(_) => break,
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                for s in element_slots(&e) {
                    fields.push(Field {
                        id,
                        label: s.label,
                        kind: s.kind,
                        value: s.value,
                    });
                    id += 1;
                }
            }
            _ => {}
        }
        buf.clear();
    }
    fields
}

fn edited_start<'a>(e: &BytesStart<'a>, base_id: usize, edits: &[Edit]) -> BytesStart<'a> {
    let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
    let mut out = BytesStart::new(name);
    // Which attrs are slots, in order, so we can map to ids.
    let slots = element_slots(e);
    let mut slot_i = 0usize;
    for a in e.attributes().flatten() {
        let key = local(a.key.as_ref());
        let orig = a.unescape_value().map(|c| c.into_owned()).unwrap_or_default();
        let is_slot = slots.get(slot_i).map(|s| s.attr == key).unwrap_or(false);
        let value = if is_slot {
            let this_id = base_id + slot_i;
            slot_i += 1;
            edits.iter().find(|ed| ed.id == this_id).map(|ed| ed.value.clone()).unwrap_or(orig)
        } else {
            orig
        };
        out.push_attribute((
            String::from_utf8_lossy(a.key.as_ref()).as_ref(),
            value.as_str(),
        ));
    }
    out
}

/// Apply edits and return the new XML (values changed, structure preserved).
pub fn apply_edits(xml: &str, edits: &[Edit]) -> Result<String, String> {
    let mut reader = Reader::from_str(xml);
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut buf = Vec::new();
    let mut id = 0usize;
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Eof) => break,
            Err(e) => return Err(e.to_string()),
            Ok(Event::Start(e)) => {
                let n = element_slots(&e).len();
                let out = edited_start(&e, id, edits);
                id += n;
                writer.write_event(Event::Start(out)).map_err(|e| e.to_string())?;
            }
            Ok(Event::Empty(e)) => {
                let n = element_slots(&e).len();
                let out = edited_start(&e, id, edits);
                id += n;
                writer.write_event(Event::Empty(out)).map_err(|e| e.to_string())?;
            }
            Ok(ev) => writer.write_event(ev).map_err(|e| e.to_string())?,
        }
        buf.clear();
    }
    let bytes = writer.into_inner().into_inner();
    let mut s = String::from_utf8(bytes).map_err(|e| e.to_string())?;
    // Preserve a leading UTF-8 BOM if the original had one.
    if xml.starts_with('\u{feff}') && !s.starts_with('\u{feff}') {
        s.insert(0, '\u{feff}');
    }
    Ok(s)
}

/// Find a mod's settings files under `modSettings/<modName>/`.
pub fn find_files(user_dir: &Path, mod_name: &str) -> Vec<PathBuf> {
    let dir = user_dir.join("modSettings").join(mod_name);
    let mut out = Vec::new();
    if let Ok(rd) = std::fs::read_dir(&dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.extension().map(|x| x.eq_ignore_ascii_case("xml")).unwrap_or(false) {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

/// Names of mods that currently have a settings folder with XML in it.
pub fn mods_with_settings(user_dir: &Path) -> Vec<String> {
    let dir = user_dir.join("modSettings");
    let mut out = Vec::new();
    if let Ok(rd) = std::fs::read_dir(&dir) {
        for e in rd.flatten() {
            if e.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                let name = e.file_name().to_string_lossy().into_owned();
                if !find_files(user_dir, &name).is_empty() {
                    out.push(name);
                }
            }
        }
    }
    out
}

pub fn load_file(path: &Path) -> Result<SettingsFile, String> {
    let raw = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    Ok(SettingsFile {
        path: path.to_string_lossy().into_owned(),
        name: path.file_name().map(|f| f.to_string_lossy().into_owned()).unwrap_or_default(),
        fields: parse(&raw),
        raw,
    })
}

/// Write edits back, backing up the original to `<file>.bak` first.
pub fn save(path: &Path, edits: &[Edit]) -> Result<(), String> {
    let raw = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let updated = apply_edits(&raw, edits)?;
    let _ = std::fs::copy(path, path.with_extension("xml.bak"));
    std::fs::write(path, updated).map_err(|e| e.to_string())
}

/// Overwrite a settings file with raw content (the escape hatch), backing up first.
pub fn save_raw(path: &Path, content: &str) -> Result<(), String> {
    let _ = std::fs::copy(path, path.with_extension("xml.bak"));
    std::fs::write(path, content).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "<settings>\n    <setting name=\"MAX\" integer=\"1\"/>\n    <setting name=\"ON\" boolean=\"true\"/>\n</settings>";

    #[test]
    fn parses_typed_fields() {
        let f = parse(SAMPLE);
        assert_eq!(f.len(), 2);
        assert_eq!(f[0].label, "MAX");
        assert_eq!(f[0].kind, "int");
        assert_eq!(f[0].value, "1");
        assert_eq!(f[1].kind, "bool");
        assert_eq!(f[1].value, "true");
    }

    #[test]
    fn edits_only_change_targeted_values() {
        let edits = vec![
            Edit { id: 0, value: "5".into() },
            Edit { id: 1, value: "false".into() },
        ];
        let out = apply_edits(SAMPLE, &edits).unwrap();
        assert!(out.contains("integer=\"5\""));
        assert!(out.contains("boolean=\"false\""));
        assert!(out.contains("name=\"MAX\""));
        // Re-parsing reflects the edits.
        let f = parse(&out);
        assert_eq!(f[0].value, "5");
        assert_eq!(f[1].value, "false");
    }
}
