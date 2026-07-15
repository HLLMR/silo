//! Path-addressable read/write for structured config XML (game.xml,
//! careerSavegame.xml). Unlike `settings_form` (which auto-discovers typed
//! attributes), this edits *specific known* values chosen by a curated schema in
//! the UI — so we can present friendly labels, option lists, and presets.
//!
//! Paths are dot-separated element names from the root, with an optional `@attr`
//! suffix for an attribute value:
//!   "game.graphic.scalability.shadowMapSize"   (leaf element text)
//!   "game.graphic.display.vsync@adaptive"      (attribute)
//!
//! Write-back re-emits the document as read, changing only targeted values.

use quick_xml::events::{BytesStart, BytesText, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Cursor;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edit {
    pub path: String,
    pub value: String,
}

fn local(name: &[u8]) -> String {
    let s = String::from_utf8_lossy(name);
    s.rsplit(':').next().unwrap_or(&s).to_string()
}

fn split_attr(path: &str) -> (&str, Option<&str>) {
    match path.split_once('@') {
        Some((el, attr)) => (el, Some(attr)),
        None => (path, None),
    }
}

/// Read the values at the given paths (missing paths are simply absent).
pub fn get_values(xml: &str, paths: &[String]) -> HashMap<String, String> {
    // Partition targets into element-text vs attribute lookups.
    let mut want_text: HashMap<String, String> = HashMap::new(); // elemPath -> resultKey
    let mut want_attr: HashMap<(String, String), String> = HashMap::new(); // (elemPath, attr) -> key
    for p in paths {
        let (el, attr) = split_attr(p);
        match attr {
            Some(a) => {
                want_attr.insert((el.to_string(), a.to_string()), p.clone());
            }
            None => {
                want_text.insert(el.to_string(), p.clone());
            }
        }
    }

    let mut out = HashMap::new();
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    let mut stack: Vec<String> = Vec::new();

    let check_attrs = |e: &BytesStart, path: &str, out: &mut HashMap<String, String>| {
        for a in e.attributes().flatten() {
            let key = local(a.key.as_ref());
            if let Some(rk) = want_attr.get(&(path.to_string(), key)) {
                if let Ok(v) = a.unescape_value() {
                    out.insert(rk.clone(), v.into_owned());
                }
            }
        }
    };

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Eof) | Err(_) => break,
            Ok(Event::Start(e)) => {
                stack.push(local(e.name().as_ref()));
                check_attrs(&e, &stack.join("."), &mut out);
            }
            Ok(Event::Empty(e)) => {
                stack.push(local(e.name().as_ref()));
                check_attrs(&e, &stack.join("."), &mut out);
                stack.pop();
            }
            Ok(Event::End(_)) => {
                stack.pop();
            }
            Ok(Event::Text(t)) => {
                let path = stack.join(".");
                if let Some(rk) = want_text.get(&path) {
                    if let Ok(v) = t.unescape() {
                        let v = v.trim();
                        if !v.is_empty() {
                            out.insert(rk.clone(), v.to_string());
                        }
                    }
                }
            }
            _ => {}
        }
        buf.clear();
    }
    out
}

fn rebuild_start<'a>(e: &BytesStart<'a>, path: &str, edits: &[Edit]) -> BytesStart<'a> {
    let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
    let mut out = BytesStart::new(name);
    for a in e.attributes().flatten() {
        let key = local(a.key.as_ref());
        let orig = a.unescape_value().map(|c| c.into_owned()).unwrap_or_default();
        let target = format!("{path}@{key}");
        let value = edits
            .iter()
            .find(|ed| ed.path == target)
            .map(|ed| ed.value.clone())
            .unwrap_or(orig);
        out.push_attribute((String::from_utf8_lossy(a.key.as_ref()).as_ref(), value.as_str()));
    }
    out
}

/// Apply edits and return the new XML (only targeted values change).
pub fn set_values(xml: &str, edits: &[Edit]) -> Result<String, String> {
    let mut reader = Reader::from_str(xml);
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut buf = Vec::new();
    let mut stack: Vec<String> = Vec::new();
    // When the previous Start matched a text-edit path, replace the next Text.
    let mut pending_text: Option<String> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Eof) => break,
            Err(e) => return Err(e.to_string()),
            Ok(Event::Start(e)) => {
                stack.push(local(e.name().as_ref()));
                let path = stack.join(".");
                let out = rebuild_start(&e, &path, edits);
                writer.write_event(Event::Start(out)).map_err(|e| e.to_string())?;
                pending_text = edits.iter().find(|ed| ed.path == path).map(|ed| ed.value.clone());
            }
            Ok(Event::Empty(e)) => {
                stack.push(local(e.name().as_ref()));
                let path = stack.join(".");
                let out = rebuild_start(&e, &path, edits);
                stack.pop();
                writer.write_event(Event::Empty(out)).map_err(|e| e.to_string())?;
            }
            Ok(Event::End(e)) => {
                stack.pop();
                pending_text = None;
                writer.write_event(Event::End(e)).map_err(|e| e.to_string())?;
            }
            Ok(Event::Text(t)) => {
                if let Some(v) = pending_text.take() {
                    writer
                        .write_event(Event::Text(BytesText::new(&v)))
                        .map_err(|e| e.to_string())?;
                } else {
                    writer.write_event(Event::Text(t)).map_err(|e| e.to_string())?;
                }
            }
            Ok(ev) => writer.write_event(ev).map_err(|e| e.to_string())?,
        }
        buf.clear();
    }
    let bytes = writer.into_inner().into_inner();
    let mut s = String::from_utf8(bytes).map_err(|e| e.to_string())?;
    if xml.starts_with('\u{feff}') && !s.starts_with('\u{feff}') {
        s.insert(0, '\u{feff}');
    }
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    const G: &str = r#"<game>
    <graphic>
        <scalability>
            <shadowMapSize>2048</shadowMapSize>
            <softShadows>true</softShadows>
            <fsr quality="1"/>
        </scalability>
    </graphic>
</game>"#;

    #[test]
    fn gets_element_text_and_attr() {
        let v = get_values(
            G,
            &[
                "game.graphic.scalability.shadowMapSize".into(),
                "game.graphic.scalability.fsr@quality".into(),
            ],
        );
        assert_eq!(v.get("game.graphic.scalability.shadowMapSize").map(|s| s.as_str()), Some("2048"));
        assert_eq!(v.get("game.graphic.scalability.fsr@quality").map(|s| s.as_str()), Some("1"));
    }

    #[test]
    fn sets_element_text_and_attr() {
        let out = set_values(
            G,
            &[
                Edit { path: "game.graphic.scalability.shadowMapSize".into(), value: "4096".into() },
                Edit { path: "game.graphic.scalability.softShadows".into(), value: "false".into() },
                Edit { path: "game.graphic.scalability.fsr@quality".into(), value: "3".into() },
            ],
        )
        .unwrap();
        assert!(out.contains("<shadowMapSize>4096</shadowMapSize>"));
        assert!(out.contains("<softShadows>false</softShadows>"));
        assert!(out.contains("quality=\"3\""));
        // Untouched structure preserved.
        assert!(out.contains("<graphic>"));
    }
}
