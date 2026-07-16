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
}

#[derive(Debug, Clone, Deserialize)]
struct BrowseResponse {
    mods: Vec<BrowseMod>,
}

/// A per-source pointer (from `GET /mods/:id`). The download url lives here.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModSource {
    pub source: String,
    pub source_url: String,
    #[serde(default)]
    pub download_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ModDetail {
    #[serde(default)]
    sources: Vec<ModSource>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub mods: u64,
    pub sources: u64,
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

/// Catalog counts. The API returns counts as strings (bigint) — parse leniently.
pub fn stats(base: &str) -> Result<Stats, String> {
    let resp = get(&format!("{}/stats", base.trim_end_matches('/')))?;
    let v: serde_json::Value = resp.into_json().map_err(|e| e.to_string())?;
    Ok(Stats {
        mods: as_u64(&v["mods"]),
        sources: as_u64(&v["sources"]),
    })
}

/// Resolve a mod's best downloadable source into (url, filename). Prefers GitHub
/// (author-sanctioned release) over ModHub CDN, but takes whatever has a url.
pub fn resolve_download(base: &str, id: &str) -> Result<(String, String), String> {
    let resp = get(&format!("{}/mods/{}", base.trim_end_matches('/'), id))?;
    let detail: ModDetail = resp.into_json().map_err(|e| e.to_string())?;

    let with_url: Vec<&ModSource> = detail
        .sources
        .iter()
        .filter(|s| s.download_url.as_deref().is_some_and(|u| !u.is_empty()))
        .collect();
    let pick = with_url
        .iter()
        .find(|s| s.source == "github")
        .or_else(|| with_url.first())
        .ok_or_else(|| "No downloadable source for this mod".to_string())?;

    let url = pick.download_url.clone().unwrap();
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
