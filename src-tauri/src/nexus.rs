//! Nexus Mods source integration for the detail-drawer card. Same posture as the
//! GitHub card: reads are public, and the *action* (endorse) runs through the user's
//! own Nexus personal API key and lands on Nexus's servers — Silo only reflects state.
//!
//! - Read (keyless): v2 GraphQL. A single mod's endorsement/download counts need
//!   `gameId` + `modId` together ("gameId is required when filtering by modId").
//! - Auth: the v1 API takes a personal API key in the `apikey` header (no OAuth).
//!   Verified against the official spec (securityDefinitions.accountId, header `apikey`).
//! - Act: `POST /v1/games/{domain}/mods/{id}/endorse.json` | `abstain.json`.

use serde::Serialize;

const V2: &str = "https://api.nexusmods.com/v2/graphql";
const V1: &str = "https://api.nexusmods.com/v1";
const UA: &str = "Silo-FS25-Mod-Manager";
/// FS25 on Nexus: domain `farmingsimulator25`, numeric game id `7052`.
const FS25_GAME_ID: &str = "7052";
pub const FS25_DOMAIN: &str = "farmingsimulator25";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NexusMod {
    pub mod_id: u64,
    pub name: Option<String>,
    pub endorsements: u64,
    pub downloads: u64,
    /// Your endorsement state, when a key is connected: Some(true)=endorsed,
    /// Some(false)=not endorsed/abstained, None=unknown (no key or unreadable).
    pub you_endorsed: Option<bool>,
}

fn nexus_err(e: ureq::Error) -> String {
    match e {
        ureq::Error::Status(401, _) => "Nexus rejected the API key — reconnect".to_string(),
        ureq::Error::Status(403, _) => {
            "Nexus refused this action (key lacks permission, or endorsement rules not met \
             — you must have downloaded the mod first)"
                .to_string()
        }
        ureq::Error::Status(404, _) => "Mod not found on Nexus".to_string(),
        ureq::Error::Status(code, _) => format!("Nexus returned {code}"),
        other => other.to_string(),
    }
}

/// Keyless public counts for one mod via v2 GraphQL.
fn read_counts(mod_id: u64) -> Result<(Option<String>, u64, u64), String> {
    let body = ureq::json!({
        "query": "query($f:ModsFilter){mods(filter:$f,count:1){nodes{modId name endorsements downloads}}}",
        "variables": { "f": {
            "gameId": [{ "value": FS25_GAME_ID, "op": "EQUALS" }],
            "modId":  [{ "value": mod_id.to_string(), "op": "EQUALS" }]
        }}
    });
    let resp = ureq::post(V2)
        .set("User-Agent", UA)
        .set("Content-Type", "application/json")
        .send_json(body)
        .map_err(nexus_err)?;
    let v: serde_json::Value = resp.into_json().map_err(|e| e.to_string())?;
    let node = &v["data"]["mods"]["nodes"][0];
    if node.is_null() {
        return Err("Mod not found on Nexus".into());
    }
    Ok((
        node["name"].as_str().map(String::from),
        node["endorsements"].as_u64().unwrap_or(0),
        node["downloads"].as_u64().unwrap_or(0),
    ))
}

/// Whether the user has endorsed this mod, read from their endorsements list. Parsed
/// defensively (the public spec omits the response schema): match this mod_id and a
/// status that reads as endorsed. Returns None if it can't be determined.
fn your_endorsed(mod_id: u64, key: &str) -> Option<bool> {
    let resp = ureq::get(&format!("{V1}/user/endorsements.json"))
        .set("User-Agent", UA)
        .set("Accept", "application/json")
        .set("apikey", key)
        .call()
        .ok()?;
    let v: serde_json::Value = resp.into_json().ok()?;
    // Response may be a bare array or { endorsements: [...] }.
    let arr = v.as_array().or_else(|| v["endorsements"].as_array())?;
    for e in arr {
        let id = e["mod_id"].as_u64().or_else(|| e["modId"].as_u64());
        if id != Some(mod_id) {
            continue;
        }
        let domain = e["domain_name"].as_str().or_else(|| e["domainName"].as_str());
        if matches!(domain, Some(d) if d != FS25_DOMAIN) {
            continue;
        }
        let status = e["status"].as_str().or_else(|| e["endorse_status"].as_str()).unwrap_or("");
        if status.to_lowercase().contains("endors") {
            return Some(true);
        }
    }
    // The list parsed but this mod isn't among the endorsed → not currently endorsed.
    Some(false)
}

/// Public counts for one mod, plus your endorse state when a key is supplied.
pub fn mod_stats(mod_id: u64, key: Option<&str>) -> Result<NexusMod, String> {
    let (name, endorsements, downloads) = read_counts(mod_id)?;
    let you_endorsed = key.and_then(|k| your_endorsed(mod_id, k));
    Ok(NexusMod { mod_id, name, endorsements, downloads, you_endorsed })
}

/// Full mod body from the keyless v2 GraphQL `description` (BBCode + <br/>), cleaned
/// to readable plain text. This is where advanced mods put keybinds, capacities, and
/// filltype notes — worth having a click away.
pub fn mod_description(mod_id: u64) -> Result<String, String> {
    let body = ureq::json!({
        "query": "query($f:ModsFilter){mods(filter:$f,count:1){nodes{description}}}",
        "variables": { "f": {
            "gameId": [{ "value": FS25_GAME_ID, "op": "EQUALS" }],
            "modId":  [{ "value": mod_id.to_string(), "op": "EQUALS" }]
        }}
    });
    let resp = ureq::post(V2)
        .set("User-Agent", UA)
        .set("Content-Type", "application/json")
        .send_json(body)
        .map_err(nexus_err)?;
    let v: serde_json::Value = resp.into_json().map_err(|e| e.to_string())?;
    let raw = v["data"]["mods"]["nodes"][0]["description"].as_str().unwrap_or("");
    Ok(clean_bbcode(raw))
}

/// Strip BBCode tags and `<br/>`, decode a few HTML entities, tidy blank lines — enough
/// to make a Nexus description readable as plain text without a BBCode renderer.
fn clean_bbcode(s: &str) -> String {
    let s = s
        .replace("<br />", "\n")
        .replace("<br/>", "\n")
        .replace("<br>", "\n");
    // Drop anything between [ and ] (BBCode tags), keeping the inner text.
    let mut stripped = String::with_capacity(s.len());
    let mut depth: u32 = 0;
    for ch in s.chars() {
        match ch {
            '[' => depth += 1,
            ']' if depth > 0 => depth -= 1,
            _ if depth == 0 => stripped.push(ch),
            _ => {}
        }
    }
    let stripped = stripped
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ");
    // Collapse runs of blank lines to at most one, trim trailing spaces.
    let mut out = String::new();
    let mut blanks = 0;
    for line in stripped.lines() {
        let t = line.trim_end();
        if t.trim().is_empty() {
            blanks += 1;
            if blanks <= 1 {
                out.push('\n');
            }
        } else {
            blanks = 0;
            out.push_str(t);
            out.push('\n');
        }
    }
    out.trim().to_string()
}

/// Verify a personal API key and return the account name.
pub fn validate_key(key: &str) -> Result<String, String> {
    let resp = ureq::get(&format!("{V1}/users/validate.json"))
        .set("User-Agent", UA)
        .set("Accept", "application/json")
        .set("apikey", key)
        .call()
        .map_err(nexus_err)?;
    let v: serde_json::Value = resp.into_json().map_err(|e| e.to_string())?;
    let name = v["name"].as_str().unwrap_or("").to_string();
    if name.is_empty() {
        return Err("Nexus did not recognize that API key".into());
    }
    Ok(name)
}

/// Endorse (on=true) or abstain (on=false). `version` is optional formData; Nexus may
/// require it, so pass the mod's version through when known.
pub fn set_endorse(
    mod_id: u64,
    key: &str,
    on: bool,
    version: Option<&str>,
) -> Result<bool, String> {
    let action = if on { "endorse" } else { "abstain" };
    let url = format!("{V1}/games/{FS25_DOMAIN}/mods/{mod_id}/{action}.json");
    let form: Vec<(&str, &str)> = match version {
        Some(v) if !v.trim().is_empty() => vec![("version", v.trim())],
        _ => vec![],
    };
    ureq::post(&url)
        .set("User-Agent", UA)
        .set("Accept", "application/json")
        .set("apikey", key)
        .send_form(&form)
        .map_err(nexus_err)?;
    Ok(on)
}

/// Parse a Nexus mod id out of a source URL (…/farmingsimulator25/mods/12345).
pub fn parse_mod_id(url: &str) -> Option<u64> {
    let i = url.find("/mods/")? + "/mods/".len();
    let digits: String = url[i..].chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_mod_id() {
        assert_eq!(
            parse_mod_id("https://www.nexusmods.com/farmingsimulator25/mods/123"),
            Some(123)
        );
        assert_eq!(
            parse_mod_id("https://nexusmods.com/games/farmingsimulator25/mods/4567?tab=files"),
            Some(4567)
        );
        assert_eq!(parse_mod_id("https://example.com/nope"), None);
    }

    #[test]
    fn cleans_bbcode() {
        let raw = "[b][size=3]Title[/size]\n<br />\n<br />Keybinds:\n<br />[/b]\n<br />CTRL+ALT+0 for $1M";
        let out = clean_bbcode(raw);
        assert!(out.starts_with("Title"), "got: {out:?}");
        assert!(out.contains("Keybinds:"));
        assert!(out.contains("CTRL+ALT+0 for $1M"));
        assert!(!out.contains('['), "tags not stripped: {out:?}");
        assert!(!out.contains("<br"), "br not stripped: {out:?}");
    }
}
