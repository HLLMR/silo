//! Collections — a portable, shareable *list* of mods (never the ZIPs themselves).
//!
//! A collection is a superset of a loadout (`LoadoutFile`): where a loadout is just
//! a name + a set of tech-names for local use, a collection carries enough per-mod
//! detail (source, canonical provenance hash, version) that a recipient can resolve,
//! fetch, and verify each mod through Silo's existing catalog → download → verify
//! pipeline. Phase 1 shares it through the user's own GitHub as a secret gist; this
//! module is only the on-the-wire format + the small pure helpers around it (schema
//! validation, gist-URL parsing). The export/import commands live elsewhere.
//!
//! Two hash spaces coexist deliberately (see `mpsync` and `provenance`): a collection
//! carries the canonical `manifestHash` (cross-source, order-independent) as its trust
//! field, so a recipient can fetch a *verified* build from any source — not only
//! byte-match the curator's exact zip. mpsync's whole-zip MD5 stays for the separate
//! host↔joiner multiplayer diff.

use serde::{Deserialize, Serialize};

/// The current wire schema. Bumped on a breaking format change; readers accept any
/// document under the `silo.collection/` family (forward-lenient — unknown-but-parseable
/// minor versions still load, since every field beyond the required three is optional).
pub const SCHEMA: &str = "silo.collection/1";
const SCHEMA_PREFIX: &str = "silo.collection/";

/// The canonical file name a collection is stored under inside a gist (or, later, a repo).
pub const FILE_NAME: &str = "silo-collection.json";

/// A shareable collection. Distinct from `LoadoutFile` (which stays the local,
/// bare-tech-name `.silo` interchange) — a collection down-imports to a loadout by
/// dropping everything but the tech-names.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    /// Format tag, e.g. `"silo.collection/1"`. Validated on read.
    pub schema: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Curator's GitHub login, stamped at export. (Distinct from any per-mod author.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// RFC 3339, client-stamped at export.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Optional savegame/map pin (for MP or a save-bound share).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savegame: Option<SaveBinding>,
    pub mods: Vec<CollectionMod>,
}

/// One mod in a collection. Only `tech_name` is required; the rest is populated from
/// the catalog record + provenance at export time, and is what makes the import
/// fix-list *actionable* (resolve a source, verify against a hash) rather than a bare
/// name list.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionMod {
    pub tech_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Preferred source: `"github" | "modhub" | "nexus"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Page/release URL — used for the open-page branch when a source can't be installed directly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    /// Canonical provenance hash (cross-source), when known — the trust field on import.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_hash: Option<String>,
    /// Whether a direct in-app install is possible; `false`/absent → the importer opens the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installable: Option<bool>,
    /// SiloAPI mod id, to skip a lookup on resolve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
}

/// Optional savegame binding. Enough to re-associate without pinning a local path.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveBinding {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_title: Option<String>,
}

impl Collection {
    /// The tech-names in this collection, in order — the down-import to a plain loadout.
    pub fn tech_names(&self) -> Vec<String> {
        self.mods.iter().map(|m| m.tech_name.clone()).collect()
    }
}

/// Serialize a collection to the pretty JSON we write to a gist / file.
pub fn to_json(c: &Collection) -> Result<String, String> {
    serde_json::to_string_pretty(c).map_err(|e| e.to_string())
}

/// Parse + validate an untrusted collection document. Rejects anything that isn't a
/// Silo collection (wrong/missing schema or malformed JSON) with one user-facing message.
pub fn parse(json: &str) -> Result<Collection, String> {
    let c: Collection = serde_json::from_str(json)
        .map_err(|_| "That file isn't a valid Silo collection".to_string())?;
    if !c.schema.starts_with(SCHEMA_PREFIX) {
        return Err("That file isn't a valid Silo collection".to_string());
    }
    Ok(c)
}

/// A GitHub gist id is an opaque hex string. Historically 32 chars; we accept a lenient
/// hex range so the parse survives GitHub changing the length.
fn is_gist_id(s: &str) -> bool {
    (20..=40).contains(&s.len()) && s.bytes().all(|b| b.is_ascii_hexdigit())
}

/// Pull a gist id out of whatever the user pasted: a bare id, a
/// `gist.github.com/{user}/{id}` or `/{id}` link, a `gist.githubusercontent.com` raw
/// URL, or an `api.github.com/gists/{id}` URL. Returns `None` if no gist id is present.
pub fn parse_gist_ref(input: &str) -> Option<String> {
    let s = input.trim();
    // Drop any query/fragment (e.g. `#file-silo-collection-json`) and trailing slash.
    let s = s
        .split(['#', '?'])
        .next()
        .unwrap_or(s)
        .trim_end_matches('/');

    // A bare id needs no host.
    if is_gist_id(s) {
        return Some(s.to_string());
    }

    let rest = s
        .strip_prefix("https://")
        .or_else(|| s.strip_prefix("http://"))
        .unwrap_or(s);
    let mut it = rest.splitn(2, '/');
    let host = it.next()?.to_ascii_lowercase();
    let path = it.next().unwrap_or("");
    let segs: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();

    let candidate = match host.as_str() {
        // /gists/{id}
        "api.github.com" => segs
            .iter()
            .position(|&p| p == "gists")
            .and_then(|i| segs.get(i + 1).copied()),
        // /{id}  or  /{user}/{id}[/...]
        "gist.github.com" => match segs.len() {
            0 => None,
            1 => Some(segs[0]),
            _ => Some(segs[1]),
        },
        // /{user}/{id}/raw/...
        "gist.githubusercontent.com" => segs.get(1).copied(),
        _ => None,
    };

    candidate.filter(|c| is_gist_id(c)).map(|c| c.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Collection {
        Collection {
            schema: SCHEMA.to_string(),
            name: "My Server Pack".to_string(),
            description: Some("Weekend co-op set".to_string()),
            author: Some("hllmr".to_string()),
            created_at: Some("2026-08-01T00:00:00Z".to_string()),
            savegame: Some(SaveBinding {
                name: "Co-op Farm".to_string(),
                map_title: Some("Riverbend".to_string()),
            }),
            mods: vec![
                CollectionMod {
                    tech_name: "FS25_Foo".to_string(),
                    version: Some("1.2.0".to_string()),
                    source: Some("github".to_string()),
                    source_url: Some("https://github.com/x/FS25_Foo".to_string()),
                    manifest_hash: Some("a3944f99".to_string()),
                    installable: Some(true),
                    catalog_id: Some("42".to_string()),
                },
                CollectionMod {
                    tech_name: "FS25_Bar".to_string(),
                    version: None,
                    source: Some("modhub".to_string()),
                    source_url: Some("https://www.farming-simulator.com/mod.php?id=1".to_string()),
                    manifest_hash: None,
                    installable: Some(false),
                    catalog_id: None,
                },
            ],
        }
    }

    #[test]
    fn round_trips_through_json() {
        let c = sample();
        let json = to_json(&c).unwrap();
        let back = parse(&json).unwrap();
        assert_eq!(c, back);
    }

    #[test]
    fn omits_absent_optional_fields() {
        // A minimal doc (only the required three) must serialize without null noise
        // and parse back with the optionals as None.
        let c = Collection {
            schema: SCHEMA.to_string(),
            name: "Bare".to_string(),
            description: None,
            author: None,
            created_at: None,
            savegame: None,
            mods: vec![CollectionMod {
                tech_name: "FS25_Only".to_string(),
                version: None,
                source: None,
                source_url: None,
                manifest_hash: None,
                installable: None,
                catalog_id: None,
            }],
        };
        let json = to_json(&c).unwrap();
        assert!(
            !json.contains("null"),
            "optional fields should be omitted, not null"
        );
        assert!(!json.contains("description"));
        assert_eq!(parse(&json).unwrap(), c);
    }

    #[test]
    fn down_imports_to_tech_names() {
        assert_eq!(sample().tech_names(), vec!["FS25_Foo", "FS25_Bar"]);
    }

    #[test]
    fn rejects_non_collection_json() {
        // A loadout file (no schema field) is not a collection.
        let loadout = r#"{ "silo": 1, "name": "x", "mods": ["FS25_A"] }"#;
        assert!(parse(loadout).is_err());
        // Wrong family.
        let wrong = r#"{ "schema": "silo.loadout/1", "name": "x", "mods": [] }"#;
        assert!(parse(wrong).is_err());
        // Garbage.
        assert!(parse("not json").is_err());
    }

    #[test]
    fn accepts_forward_minor_schema() {
        // A future minor under the same family still loads (forward-lenient).
        let doc = r#"{ "schema": "silo.collection/1.1", "name": "x", "mods": [] }"#;
        assert!(parse(doc).is_ok());
    }

    #[test]
    fn parses_gist_ref_from_every_shape() {
        let id = "aa11bb22cc33dd44ee55ff6677889900";
        // bare id
        assert_eq!(parse_gist_ref(id).as_deref(), Some(id));
        // canonical user/id
        assert_eq!(
            parse_gist_ref(&format!("https://gist.github.com/hllmr/{id}")).as_deref(),
            Some(id)
        );
        // id-only URL
        assert_eq!(
            parse_gist_ref(&format!("https://gist.github.com/{id}")).as_deref(),
            Some(id)
        );
        // with fragment + trailing slash
        assert_eq!(
            parse_gist_ref(&format!(
                "https://gist.github.com/hllmr/{id}/#file-silo-collection-json"
            ))
            .as_deref(),
            Some(id)
        );
        // api URL
        assert_eq!(
            parse_gist_ref(&format!("https://api.github.com/gists/{id}")).as_deref(),
            Some(id)
        );
        // raw usercontent URL
        assert_eq!(
            parse_gist_ref(&format!(
                "https://gist.githubusercontent.com/hllmr/{id}/raw/abc/silo-collection.json"
            ))
            .as_deref(),
            Some(id)
        );
    }

    #[test]
    fn rejects_non_gist_input() {
        assert_eq!(parse_gist_ref("https://gist.github.com/hllmr"), None); // user page, no id
        assert_eq!(parse_gist_ref("https://github.com/HLLMR/silo"), None); // not a gist
        assert_eq!(parse_gist_ref("hello world"), None);
        assert_eq!(parse_gist_ref("short123"), None); // too short to be an id
    }
}
