//! Read the authoritative FS store category from a mod's storeItem XML. A vehicle/
//! tool/placeable XML carries `<storeData><category>tractorsM</category>…`, which is
//! GIANTS' own taxonomy — far more reliable than guessing from names.

use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::io::Read;
use std::path::Path;

/// Read the `<storeData><category>` of the first storeItem, if any.
pub fn first_store_category(mod_path: &Path, kind: &str, store_files: &[String]) -> Option<String> {
    let first = store_files.first()?;
    let xml = read_member(mod_path, kind, first)?;
    parse_store_category(&xml)
}

fn read_member(mod_path: &Path, kind: &str, member: &str) -> Option<String> {
    let member = member.replace('\\', "/");
    match kind {
        "zip" => {
            let f = std::fs::File::open(mod_path).ok()?;
            let mut ar = zip::ZipArchive::new(f).ok()?;
            let mut entry = ar.by_name(&member).ok()?;
            let mut s = String::new();
            entry.read_to_string(&mut s).ok()?;
            Some(s)
        }
        _ => std::fs::read_to_string(mod_path.join(&member)).ok(),
    }
}

/// Extract the text of `<category>` whose parent is `<storeData>`.
fn parse_store_category(xml: &str) -> Option<String> {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    let mut stack: Vec<String> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Eof) | Err(_) => break,
            Ok(Event::Start(e)) => {
                stack.push(String::from_utf8_lossy(e.local_name().as_ref()).into_owned());
            }
            Ok(Event::End(_)) => {
                stack.pop();
            }
            Ok(Event::Text(t)) => {
                let last = stack.last().map(String::as_str).unwrap_or("");
                let parent = stack.iter().rev().nth(1).map(String::as_str).unwrap_or("");
                if last == "category" && parent == "storeData" {
                    if let Ok(txt) = t.unescape() {
                        let v = txt.trim().to_string();
                        if !v.is_empty() {
                            return Some(v);
                        }
                    }
                }
            }
            _ => {}
        }
        buf.clear();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_category() {
        let xml = r#"<vehicle><storeData><name>X</name><category>tractorsM</category></storeData></vehicle>"#;
        assert_eq!(parse_store_category(xml).as_deref(), Some("tractorsM"));
    }
}
