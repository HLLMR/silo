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

const UA: &str = "Silo-FS25-Mod-Manager";

/// Fetch the latest release for `owner/repo`. A token (when present) raises the
/// rate limit to 5000/hr and allows private repos.
pub fn latest_release(owner: &str, repo: &str, token: Option<&str>) -> Result<ReleaseInfo, String> {
    let url = format!("https://api.github.com/repos/{owner}/{repo}/releases/latest");
    let mut req = ureq::get(&url)
        .set("User-Agent", UA)
        .set("Accept", "application/vnd.github+json");
    if let Some(t) = token {
        req = req.set("Authorization", &format!("Bearer {t}"));
    }
    let resp = req
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
pub fn check(owner: &str, repo: &str, current: &str, token: Option<&str>) -> Result<UpdateInfo, String> {
    let release = latest_release(owner, repo, token)?;
    Ok(UpdateInfo {
        has_update: is_newer(&release.tag, current),
        current: current.to_string(),
        release,
    })
}

// ── OAuth Device Flow (RFC 8628) — no client secret, ideal for desktop ──

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceCode {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub interval: u64,
    pub expires_in: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PollResult {
    /// "ok" | "pending" | "slow_down" | "expired" | "denied" | "error"
    pub status: String,
    pub token: Option<String>,
    pub error: Option<String>,
}

/// Step 1: request a device + user code for the given OAuth App client id.
pub fn device_start(client_id: &str) -> Result<DeviceCode, String> {
    let resp = ureq::post("https://github.com/login/device/code")
        .set("Accept", "application/json")
        .set("User-Agent", UA)
        .send_form(&[("client_id", client_id), ("scope", "read:user")])
        .map_err(|e| e.to_string())?;
    let v: serde_json::Value = resp.into_json().map_err(|e| e.to_string())?;
    if let Some(err) = v["error"].as_str() {
        return Err(format!(
            "{}: {}",
            err,
            v["error_description"].as_str().unwrap_or("device code request failed")
        ));
    }
    Ok(DeviceCode {
        device_code: v["device_code"].as_str().unwrap_or("").to_string(),
        user_code: v["user_code"].as_str().unwrap_or("").to_string(),
        verification_uri: v["verification_uri"].as_str().unwrap_or("https://github.com/login/device").to_string(),
        interval: v["interval"].as_u64().unwrap_or(5),
        expires_in: v["expires_in"].as_u64().unwrap_or(900),
    })
}

/// Step 2 (polled): exchange the device code for a token once the user approves.
pub fn device_poll(client_id: &str, device_code: &str) -> Result<PollResult, String> {
    let resp = ureq::post("https://github.com/login/oauth/access_token")
        .set("Accept", "application/json")
        .set("User-Agent", UA)
        .send_form(&[
            ("client_id", client_id),
            ("device_code", device_code),
            ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
        ])
        .map_err(|e| e.to_string())?;
    let v: serde_json::Value = resp.into_json().map_err(|e| e.to_string())?;
    if let Some(tok) = v["access_token"].as_str() {
        return Ok(PollResult { status: "ok".into(), token: Some(tok.to_string()), error: None });
    }
    let err = v["error"].as_str().unwrap_or("error");
    let status = match err {
        "authorization_pending" => "pending",
        "slow_down" => "slow_down",
        "expired_token" => "expired",
        "access_denied" => "denied",
        _ => "error",
    };
    Ok(PollResult { status: status.into(), token: None, error: Some(err.to_string()) })
}

/// The authenticated user's login name (verifies a token).
pub fn whoami(token: &str) -> Result<String, String> {
    let resp = ureq::get("https://api.github.com/user")
        .set("Accept", "application/vnd.github+json")
        .set("User-Agent", UA)
        .set("Authorization", &format!("Bearer {token}"))
        .call()
        .map_err(|e| e.to_string())?;
    let v: serde_json::Value = resp.into_json().map_err(|e| e.to_string())?;
    Ok(v["login"].as_str().unwrap_or("").to_string())
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
