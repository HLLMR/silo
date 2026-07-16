//! Client for SiloAPI — our canonical FS mod database (https://silo-api.hllmr.com).
//! This is the "Browse" tab's backend: search the catalog, read a mod's cross-source
//! pointers, and install a browsed mod's .zip straight into the library.
//!
//! All network stays in Rust (the webview CSP blocks HTTP). The base URL is
//! overridable via the `siloapi_base` app_setting so a self-hoster can repoint it.

use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

const UA: &str = "Silo-FS25-Mod-Manager";
pub const DEFAULT_BASE: &str = "https://silo-api.hllmr.com";

/// A catalog row as returned by `GET /mods`. Fields mirror the API (camelCase).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseMod {
    pub id: String,
    #[serde(default)]
    pub tech_name: Option<String>,
    pub title: String,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub image_url: Option<String>,
    #[serde(default)]
    pub latest_version: Option<String>,
    #[serde(default)]
    pub trust_score: Option<i64>,
    #[serde(default)]
    pub updated_at: Option<String>,
    /// Every place this mod can be got from, best-first. Drives the per-source buttons.
    #[serde(default)]
    pub sources: Vec<ModSource>,
    /// Where to send the user when nothing is directly installable.
    #[serde(default)]
    pub page_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct BrowseResponse {
    mods: Vec<BrowseMod>,
}

/// One place a mod can be got from, as the API reports it. The API decides
/// `installable` (ModHub's CDN 403s hotlinked GETs; Nexus gates downloads), so the
/// client never re-derives that policy — it just renders a button per source.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModSource {
    pub source: String,
    pub source_url: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub installable: bool,
    /// Only present when `installable` — the API strips URLs it knows won't serve us.
    #[serde(default)]
    pub download_url: Option<String>,
}

/// Full record from `GET /mods/:id` — the catalog's view of one mod, including every
/// source it was seen on. Powers the Browse tab's detail drawer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModDetail {
    pub id: String,
    #[serde(default)]
    pub tech_name: Option<String>,
    pub title: String,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub image_url: Option<String>,
    #[serde(default)]
    pub latest_version: Option<String>,
    #[serde(default)]
    pub trust_score: Option<i64>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub sources: Vec<ModSource>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub mods: u64,
    pub sources: u64,
}

/// A catalog category and how many mods carry it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryCount {
    pub category: String,
    pub count: u64,
}

#[derive(Debug, Deserialize)]
struct CategoriesResponse {
    #[serde(default)]
    categories: Vec<CategoryCount>,
}

/// A best-download pointer from the batch lookup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Download {
    pub source: String,
    pub url: String,
}

/// One row from `POST /mods/lookup`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LookupResult {
    pub tech_name: Option<String>,
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub latest_version: Option<String>,
    #[serde(default)]
    pub download: Option<Download>,
}

#[derive(Debug, Deserialize)]
struct LookupResponse {
    #[serde(default)]
    results: Vec<LookupResult>,
}

fn get(url: &str) -> Result<ureq::Response, String> {
    ureq::get(url)
        .set("User-Agent", UA)
        .call()
        .map_err(|e| match e {
            ureq::Error::Status(code, _) => format!("SiloAPI returned {code}"),
            other => format!("Could not reach SiloAPI: {other}"),
        })
}

/// Search/list the catalog. `query`/`category` are optional filters.
pub fn browse(
    base: &str,
    query: Option<&str>,
    category: Option<&str>,
    limit: u32,
    offset: u32,
) -> Result<Vec<BrowseMod>, String> {
    let mut url = format!(
        "{}/mods?limit={}&offset={}",
        base.trim_end_matches('/'),
        limit,
        offset
    );
    if let Some(q) = query.filter(|s| !s.trim().is_empty()) {
        url.push_str(&format!("&q={}", urlencode(q.trim())));
    }
    if let Some(c) = category.filter(|s| !s.trim().is_empty()) {
        url.push_str(&format!("&category={}", urlencode(c.trim())));
    }
    let resp = get(&url)?;
    let parsed: BrowseResponse = resp.into_json().map_err(|e| e.to_string())?;
    Ok(parsed.mods)
}

/// Batch version lookup by tech name. One request covers the whole library.
pub fn lookup(base: &str, tech_names: &[String]) -> Result<Vec<LookupResult>, String> {
    let resp = ureq::post(&format!("{}/mods/lookup", base.trim_end_matches('/')))
        .set("User-Agent", UA)
        .set("Content-Type", "application/json")
        .send_json(ureq::json!({ "techNames": tech_names }))
        .map_err(|e| match e {
            ureq::Error::Status(code, _) => format!("SiloAPI lookup returned {code}"),
            other => format!("Could not reach SiloAPI: {other}"),
        })?;
    let parsed: LookupResponse = resp.into_json().map_err(|e| e.to_string())?;
    Ok(parsed.results)
}

/// Catalog categories with counts (whole catalog, most-populated first).
pub fn categories(base: &str) -> Result<Vec<CategoryCount>, String> {
    let resp = get(&format!("{}/categories", base.trim_end_matches('/')))?;
    let parsed: CategoriesResponse = resp.into_json().map_err(|e| e.to_string())?;
    Ok(parsed.categories)
}

/// Catalog counts. The API returns counts as strings (bigint) — parse leniently.
pub fn stats(base: &str) -> Result<Stats, String> {
    let resp = get(&format!("{}/stats", base.trim_end_matches('/')))?;
    let v: serde_json::Value = resp.into_json().map_err(|e| e.to_string())?;
    Ok(Stats {
        mods: as_u64(&v["mods"]),
        sources: as_u64(&v["sources"]),
    })
}

/// One mod's full catalog record, including its cross-source pointers.
pub fn detail(base: &str, id: &str) -> Result<ModDetail, String> {
    let resp = get(&format!("{}/mods/{}", base.trim_end_matches('/'), id))?;
    resp.into_json().map_err(|e| e.to_string())
}

/// Resolve a download into (url, filename). `want` picks a specific source (the button
/// the user clicked); without it, the API's best-ranked installable source wins.
///
/// The API decides what's installable and strips URLs it knows won't serve us, so a
/// source with no url here is one the user must fetch from its own site.
pub fn resolve_download(
    base: &str,
    id: &str,
    want: Option<&str>,
) -> Result<(String, String), String> {
    let detail = detail(base, id)?;

    let usable = |s: &&ModSource| s.installable && s.download_url.is_some();
    let pick = match want {
        Some(name) => detail
            .sources
            .iter()
            .find(|s| s.source == name)
            .ok_or_else(|| format!("{name} doesn't list this mod"))
            .and_then(|s| {
                if usable(&s) {
                    Ok(s)
                } else {
                    Err(format!("{name} doesn't allow direct downloads — open its page instead"))
                }
            })?,
        None => detail
            .sources
            .iter()
            .find(usable)
            .ok_or_else(|| "No source allows a direct download — open the mod page".to_string())?,
    };

    let url = pick
        .download_url
        .clone()
        .ok_or_else(|| "That source has no download URL".to_string())?;
    let filename = filename_from_url(&url)
        .ok_or_else(|| "Could not derive a .zip filename from the download URL".to_string())?;
    Ok((url, filename))
}

/// Last path segment if it looks like a .zip.
fn filename_from_url(url: &str) -> Option<String> {
    let clean = url.split(['?', '#']).next().unwrap_or(url);
    let name = clean.rsplit('/').next().unwrap_or("");
    if name.to_lowercase().ends_with(".zip") && name.len() > 4 {
        Some(name.to_string())
    } else {
        None
    }
}

fn as_u64(v: &serde_json::Value) -> u64 {
    v.as_u64()
        .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
        .unwrap_or(0)
}

/// Minimal percent-encoding for query values (spaces, &, etc.).
fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// Download a resolved .zip to `dest`, streaming so the caller can report progress.
/// Validates PK magic, caps at 500 MB, writes to a `.part` file then renames on
/// success (so a failed download never leaves a half-written mod). `on_progress` is
/// called with (bytes_done, total_bytes) — total is None when the server omits
/// Content-Length. Kept Tauri-agnostic; the command layer turns it into events.
pub fn download_to<F: Fn(u64, Option<u64>)>(
    url: &str,
    dest: &std::path::Path,
    on_progress: F,
) -> Result<(), String> {
    const CAP: u64 = 500 * 1024 * 1024;
    let resp = ureq::get(url)
        .set("User-Agent", UA)
        .call()
        .map_err(|e| e.to_string())?;
    let total: Option<u64> = resp
        .header("Content-Length")
        .and_then(|s| s.parse().ok())
        .filter(|&t| t <= CAP);

    let part = dest.with_extension("zip.part");
    let mut file = std::fs::File::create(&part).map_err(|e| e.to_string())?;
    let mut reader = resp.into_reader();

    let mut buf = [0u8; 64 * 1024];
    let mut head: Vec<u8> = Vec::with_capacity(2);
    let mut done: u64 = 0;
    let mut last_emit: u64 = 0;
    on_progress(0, total);

    let result = (|| -> Result<(), String> {
        loop {
            let n = reader.read(&mut buf).map_err(|e| e.to_string())?;
            if n == 0 {
                break;
            }
            // Validate the zip's PK magic from the first two bytes.
            if head.len() < 2 {
                let need = (2 - head.len()).min(n);
                head.extend_from_slice(&buf[..need]);
                if head.len() == 2 && &head[..] != b"PK" {
                    return Err("Downloaded file is not a valid .zip".to_string());
                }
            }
            done += n as u64;
            if done > CAP {
                return Err("Download exceeds the 500 MB safety cap".to_string());
            }
            file.write_all(&buf[..n]).map_err(|e| e.to_string())?;
            // Throttle progress events to ~1 MB steps to avoid flooding the UI.
            if done - last_emit >= 1024 * 1024 {
                last_emit = done;
                on_progress(done, total);
            }
        }
        if head.len() < 2 {
            return Err("Downloaded file is not a valid .zip".to_string());
        }
        Ok(())
    })();

    if let Err(e) = result {
        drop(file);
        let _ = std::fs::remove_file(&part);
        return Err(e);
    }
    file.flush().map_err(|e| e.to_string())?;
    drop(file);
    std::fs::rename(&part, dest).map_err(|e| e.to_string())?;
    on_progress(done, total);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filename_parsing() {
        assert_eq!(
            filename_from_url("https://cdn27.giants-software.com/modHub/storage/00360212/FS25_Schmiechtal.zip"),
            Some("FS25_Schmiechtal.zip".to_string())
        );
        assert_eq!(
            filename_from_url("https://github.com/x/y/releases/download/v1/FS25_Foo.zip?raw=1"),
            Some("FS25_Foo.zip".to_string())
        );
        assert_eq!(filename_from_url("https://example.com/notazip"), None);
        assert_eq!(filename_from_url("https://example.com/.zip"), None);
    }

    #[test]
    fn urlencoding() {
        assert_eq!(urlencode("auto load"), "auto%20load");
        assert_eq!(urlencode("a&b=c"), "a%26b%3Dc");
        assert_eq!(urlencode("FS25_Mod-1.0"), "FS25_Mod-1.0");
    }

    #[test]
    fn lenient_count_parsing() {
        assert_eq!(as_u64(&serde_json::json!("215")), 215);
        assert_eq!(as_u64(&serde_json::json!(199)), 199);
        assert_eq!(as_u64(&serde_json::json!(null)), 0);
    }
}
