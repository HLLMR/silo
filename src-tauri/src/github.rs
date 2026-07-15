//! GitHub-based mod update checking. Research (2026-07) found no official GIANTS/
//! ModHub API or RSS — but a large, growing share of FS25 mods are on GitHub with a
//! strong `releases/latest` convention. This queries the GitHub REST API for a
//! repo's latest release and compares it to the installed version.
//!
//! Unauthenticated (60 req/hr per IP) is fine for on-demand checks; a PAT can lift
//! that to 5000/hr later. All network stays in Rust (the webview CSP blocks it).

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseInfo {
    pub tag: String,
    pub name: Option<String>,
    pub published_at: Option<String>,
    pub html_url: Option<String>,
    pub asset_url: Option<String>,
    pub asset_name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInfo {
    pub has_update: bool,
    pub current: String,
    pub release: ReleaseInfo,
}

/// Fetch the latest release for `owner/repo`.
pub fn latest_release(owner: &str, repo: &str) -> Result<ReleaseInfo, String> {
    let url = format!("https://api.github.com/repos/{owner}/{repo}/releases/latest");
    let resp = ureq::get(&url)
        .set("User-Agent", "Silo-FS25-Mod-Manager")
        .set("Accept", "application/vnd.github+json")
        .call()
        .map_err(|e| match e {
            ureq::Error::Status(404, _) => "No releases found for this repo".to_string(),
            ureq::Error::Status(403, _) => {
                "GitHub rate limit hit (60/hr unauthenticated) — try again later".to_string()
            }
            other => other.to_string(),
        })?;
    let v: serde_json::Value = resp.into_json().map_err(|e| e.to_string())?;

    let tag = v["tag_name"].as_str().unwrap_or("").to_string();
    if tag.is_empty() {
        return Err("Latest release has no tag".into());
    }
    // Prefer a .zip asset (the mod itself).
    let mut asset_url = None;
    let mut asset_name = None;
    if let Some(assets) = v["assets"].as_array() {
        for a in assets {
            let name = a["name"].as_str().unwrap_or("");
            if name.to_lowercase().ends_with(".zip") {
                asset_url = a["browser_download_url"].as_str().map(String::from);
                asset_name = Some(name.to_string());
                break;
            }
        }
    }
    Ok(ReleaseInfo {
        tag,
        name: v["name"].as_str().map(String::from),
        published_at: v["published_at"].as_str().map(String::from),
        html_url: v["html_url"].as_str().map(String::from),
        asset_url,
        asset_name,
    })
}

/// Compare a release tag against the installed version, tolerating a leading `v`
/// and `.`/`-`/`+` separators.
pub fn is_newer(latest_tag: &str, current: &str) -> bool {
    let norm = |s: &str| {
        s.trim_start_matches(['v', 'V'])
            .split(|c: char| c == '.' || c == '-' || c == '+' || c == '_')
            .filter_map(|p| p.parse::<u64>().ok())
            .collect::<Vec<u64>>()
    };
    let a = norm(latest_tag);
    let b = norm(current);
    for i in 0..a.len().max(b.len()) {
        let x = a.get(i).copied().unwrap_or(0);
        let y = b.get(i).copied().unwrap_or(0);
        if x != y {
            return x > y;
        }
    }
    false
}

/// Check a repo and compare to the installed version.
pub fn check(owner: &str, repo: &str, current: &str) -> Result<UpdateInfo, String> {
    let release = latest_release(owner, repo)?;
    Ok(UpdateInfo {
        has_update: is_newer(&release.tag, current),
        current: current.to_string(),
        release,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_compare() {
        assert!(is_newer("v1.2.0", "1.1.0"));
        assert!(is_newer("2.0.0.0", "1.9.9.9"));
        assert!(!is_newer("1.0.0.0", "1.0.0.0"));
        assert!(!is_newer("1.0", "1.0.0.1"));
        assert!(is_newer("v8.1.0.3", "8.1.0.2"));
    }
}
