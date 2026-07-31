//! GitHub-based mod update checking. Research (2026-07) found no official GIANTS/
//! ModHub API or RSS — but a large, growing share of FS25 mods are on GitHub with a
//! strong `releases/latest` convention. This queries the GitHub REST API for a
//! repo's latest release and compares it to the installed version.
//!
//! Unauthenticated (60 req/hr per IP) is fine for on-demand checks; a PAT can lift
//! that to 5000/hr later. All network stays in Rust (the webview CSP blocks it).

use serde::Serialize;
use std::io::Read;

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
    let resp = req.call().map_err(|e| match e {
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
            .split(['.', '-', '+', '_'])
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
pub fn check(
    owner: &str,
    repo: &str,
    current: &str,
    token: Option<&str>,
) -> Result<UpdateInfo, String> {
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
/// `scope` is space-separated OAuth scopes. Read-only identity is `read:user`;
/// to let the user star/watch through Silo we also need `public_repo` (GitHub's
/// docs: "Also required for starring public repositories").
pub fn device_start(client_id: &str, scope: &str) -> Result<DeviceCode, String> {
    let resp = ureq::post("https://github.com/login/device/code")
        .set("Accept", "application/json")
        .set("User-Agent", UA)
        .send_form(&[("client_id", client_id), ("scope", scope)])
        .map_err(|e| e.to_string())?;
    let v: serde_json::Value = resp.into_json().map_err(|e| e.to_string())?;
    if let Some(err) = v["error"].as_str() {
        return Err(format!(
            "{}: {}",
            err,
            v["error_description"]
                .as_str()
                .unwrap_or("device code request failed")
        ));
    }
    Ok(DeviceCode {
        device_code: v["device_code"].as_str().unwrap_or("").to_string(),
        user_code: v["user_code"].as_str().unwrap_or("").to_string(),
        verification_uri: v["verification_uri"]
            .as_str()
            .unwrap_or("https://github.com/login/device")
            .to_string(),
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
        return Ok(PollResult {
            status: "ok".into(),
            token: Some(tok.to_string()),
            error: None,
        });
    }
    let err = v["error"].as_str().unwrap_or("error");
    let status = match err {
        "authorization_pending" => "pending",
        "slow_down" => "slow_down",
        "expired_token" => "expired",
        "access_denied" => "denied",
        _ => "error",
    };
    Ok(PollResult {
        status: status.into(),
        token: None,
        error: Some(err.to_string()),
    })
}

/// Download a release .zip asset and install it in place at `dest`, backing up the
/// current file to `<dest>.bak` first. Overwrites the existing file (same inode) so
/// any active hardlink projection reflects the update automatically.
/// True only for GitHub-owned hosts. The token is attached ONLY to these — never to an
/// arbitrary URL handed in from the webview/catalog, so a hostile `downloadUrl` can't
/// harvest the user's credential.
pub fn is_github_host(url: &str) -> bool {
    let host = match url.split_once("://") {
        Some((_, rest)) => rest
            .split(['/', '?', '#'])
            .next()
            .unwrap_or("")
            .to_ascii_lowercase(),
        None => return false,
    };
    let host = host.split('@').next_back().unwrap_or(""); // ignore any userinfo
    let host = host.split(':').next().unwrap_or(""); // strip port
    host == "github.com"
        || host == "api.github.com"
        || host == "codeload.github.com"
        || host.ends_with(".githubusercontent.com")
}

pub fn download_zip(url: &str, token: Option<&str>, dest: &std::path::Path) -> Result<(), String> {
    let mut req = ureq::get(url).set("User-Agent", UA);
    // Attach the token ONLY to GitHub hosts. Public release assets don't need it anyway;
    // this guarantees the credential never travels to a non-GitHub URL.
    if let Some(t) = token.filter(|_| is_github_host(url)) {
        req = req.set("Authorization", &format!("Bearer {t}"));
    }
    let resp = req.call().map_err(|e| e.to_string())?;

    let mut bytes: Vec<u8> = Vec::new();
    resp.into_reader()
        .take(500 * 1024 * 1024) // 500 MB safety cap
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;

    if bytes.len() < 4 || &bytes[..2] != b"PK" {
        return Err("Downloaded file is not a valid .zip".to_string());
    }
    // Validate the WHOLE archive opens (central directory intact), not just the PK magic,
    // so a truncated or corrupt download can't overwrite a working mod with garbage.
    zip::ZipArchive::new(std::io::Cursor::new(&bytes)).map_err(|_| {
        "Downloaded file is not a valid .zip archive (corrupt or truncated)".to_string()
    })?;
    // Back up the existing file FIRST, and abort if that fails — never overwrite a mod we
    // couldn't preserve a copy of. (Previously the backup error was ignored.)
    if dest.exists() {
        std::fs::copy(dest, dest.with_extension("zip.bak")).map_err(|e| {
            format!("aborted before overwriting — couldn't back up the current file: {e}")
        })?;
    }
    std::fs::write(dest, &bytes).map_err(|e| e.to_string())
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

// ── GitHub source card: live public reads + user-owned actions ──
//
// The interaction lands on GitHub through the user's own token; Silo only reflects
// state. Reads here are fetched live when the drawer opens (per SiloAPI ENRICHMENT.md,
// rich per-source read fields are the client's job, not the aggregate API's).

/// Public repo signals for the GitHub card. `you_starred` / `you_watching` are only
/// meaningful (non-null) when a token is supplied.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoStats {
    pub full_name: String,
    pub html_url: String,
    pub stars: u64,
    pub forks: u64,
    pub watchers: u64,
    pub open_issues: u64,
    pub archived: bool,
    pub pushed_at: Option<String>,
    pub you_starred: Option<bool>,
    pub you_watching: Option<bool>,
}

fn gh_get(url: &str, token: Option<&str>) -> Result<serde_json::Value, String> {
    let mut req = ureq::get(url)
        .set("Accept", "application/vnd.github+json")
        .set("User-Agent", UA);
    if let Some(t) = token {
        req = req.set("Authorization", &format!("Bearer {t}"));
    }
    req.call()
        .map_err(gh_err)?
        .into_json()
        .map_err(|e| e.to_string())
}

/// Map a ureq error to a friendly string, calling out the scope case so the UI can
/// prompt the user to re-connect with actions enabled.
fn gh_err(e: ureq::Error) -> String {
    match e {
        ureq::Error::Status(401, _) => {
            "GitHub rejected the token — reconnect your account".to_string()
        }
        ureq::Error::Status(403, _) => {
            "This token can't perform that action — reconnect with actions enabled \
             (public_repo), or use a PAT with Starring permission"
                .to_string()
        }
        ureq::Error::Status(404, _) => "Not found on GitHub".to_string(),
        other => other.to_string(),
    }
}

/// GET /repos/{owner}/{repo} plus (when authenticated) the user's star/watch state.
pub fn repo_stats(owner: &str, repo: &str, token: Option<&str>) -> Result<RepoStats, String> {
    let v = gh_get(
        &format!("https://api.github.com/repos/{owner}/{repo}"),
        token,
    )?;
    if v["full_name"].as_str().unwrap_or("").is_empty() {
        return Err("Repo not found on GitHub".into());
    }
    let (you_starred, you_watching) = match token {
        Some(t) => (
            star_state(owner, repo, t).ok(),
            watch_state(owner, repo, t).ok(),
        ),
        None => (None, None),
    };
    Ok(RepoStats {
        full_name: v["full_name"].as_str().unwrap_or("").to_string(),
        html_url: v["html_url"].as_str().unwrap_or("").to_string(),
        stars: v["stargazers_count"].as_u64().unwrap_or(0),
        forks: v["forks_count"].as_u64().unwrap_or(0),
        // `watchers_count` mirrors stars on the REST API; `subscribers_count` is the
        // real "watching" number.
        watchers: v["subscribers_count"].as_u64().unwrap_or(0),
        open_issues: v["open_issues_count"].as_u64().unwrap_or(0),
        archived: v["archived"].as_bool().unwrap_or(false),
        pushed_at: v["pushed_at"].as_str().map(String::from),
        you_starred,
        you_watching,
    })
}

/// A GET that treats 204 as true and 404 as false — the shape GitHub uses for
/// "is this thing in the user's set?" checks.
fn present_check(url: &str, token: &str) -> Result<bool, String> {
    match ureq::get(url)
        .set("Accept", "application/vnd.github+json")
        .set("User-Agent", UA)
        .set("Authorization", &format!("Bearer {token}"))
        .call()
    {
        Ok(_) => Ok(true),
        Err(ureq::Error::Status(404, _)) => Ok(false),
        Err(e) => Err(gh_err(e)),
    }
}

/// GET /user/starred/{owner}/{repo} — 204 starred, 404 not.
pub fn star_state(owner: &str, repo: &str, token: &str) -> Result<bool, String> {
    present_check(
        &format!("https://api.github.com/user/starred/{owner}/{repo}"),
        token,
    )
}

/// GET /repos/{owner}/{repo}/subscription — 200 watching, 404 not.
pub fn watch_state(owner: &str, repo: &str, token: &str) -> Result<bool, String> {
    present_check(
        &format!("https://api.github.com/repos/{owner}/{repo}/subscription"),
        token,
    )
}

/// PUT/DELETE /user/starred/{owner}/{repo}. Needs `public_repo` (OAuth) or a PAT with
/// the Starring permission. Returns the new starred state (= `on`).
pub fn set_star(owner: &str, repo: &str, token: &str, on: bool) -> Result<bool, String> {
    let url = format!("https://api.github.com/user/starred/{owner}/{repo}");
    let req = if on {
        ureq::put(&url)
    } else {
        ureq::delete(&url)
    }
    .set("Accept", "application/vnd.github+json")
    .set("User-Agent", UA)
    .set("Content-Length", "0")
    .set("Authorization", &format!("Bearer {token}"));
    req.call().map_err(gh_err)?;
    Ok(on)
}

/// PUT (subscribe) / DELETE (unsubscribe) /repos/{owner}/{repo}/subscription.
pub fn set_watch(owner: &str, repo: &str, token: &str, on: bool) -> Result<bool, String> {
    let url = format!("https://api.github.com/repos/{owner}/{repo}/subscription");
    if on {
        ureq::put(&url)
            .set("Accept", "application/vnd.github+json")
            .set("User-Agent", UA)
            .set("Authorization", &format!("Bearer {token}"))
            .send_json(ureq::json!({ "subscribed": true, "ignored": false }))
            .map_err(gh_err)?;
    } else {
        ureq::delete(&url)
            .set("Accept", "application/vnd.github+json")
            .set("User-Agent", UA)
            .set("Content-Length", "0")
            .set("Authorization", &format!("Bearer {token}"))
            .call()
            .map_err(gh_err)?;
    }
    Ok(on)
}

/// Best-effort scan of arbitrary text (a modDesc.xml) for the first
/// `github.com/owner/repo` reference. Skips non-repo GitHub paths.
pub fn find_repo_in_text(text: &str) -> Option<(String, String)> {
    let ident = |s: &str| {
        s.chars()
            .take_while(|c| c.is_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
            .collect::<String>()
    };
    let skip = [
        "sponsors",
        "settings",
        "orgs",
        "topics",
        "about",
        "features",
        "marketplace",
    ];

    let mut from = 0;
    while let Some(i) = text[from..].find("github.com/") {
        let after = &text[from + i + "github.com/".len()..];
        let mut segs = after.split('/');
        let owner = ident(segs.next().unwrap_or(""));
        let mut repo = ident(segs.next().unwrap_or(""));
        if let Some(stripped) = repo.strip_suffix(".git") {
            repo = stripped.to_string();
        }
        if !owner.is_empty() && !repo.is_empty() && !skip.contains(&owner.as_str()) {
            return Some((owner, repo));
        }
        from += i + "github.com/".len();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_repo() {
        let t = r#"<description>Docs at https://github.com/Stephan-S/FS25_AutoDrive/wiki cool</description>"#;
        assert_eq!(
            find_repo_in_text(t),
            Some(("Stephan-S".into(), "FS25_AutoDrive".into()))
        );
        assert_eq!(
            find_repo_in_text("visit github.com/loki79uk/FS25_UniversalAutoload.git"),
            Some(("loki79uk".into(), "FS25_UniversalAutoload".into()))
        );
        assert_eq!(find_repo_in_text("no links here"), None);
        assert_eq!(
            find_repo_in_text("github.com/sponsors/foo then github.com/real/repo"),
            Some(("real".into(), "repo".into()))
        );
    }

    #[test]
    fn token_only_attaches_to_github_hosts() {
        assert!(is_github_host(
            "https://github.com/o/r/releases/download/v1/a.zip"
        ));
        assert!(is_github_host("https://api.github.com/repos/o/r"));
        assert!(is_github_host("https://objects.githubusercontent.com/x"));
        assert!(is_github_host("https://codeload.github.com/o/r/zip"));
        // Spoofing attempts must NOT be treated as GitHub.
        assert!(!is_github_host("https://github.com.evil.com/steal"));
        assert!(!is_github_host("https://evil.com/github.com/a.zip"));
        assert!(!is_github_host("https://raw.githubusercontent.example/x"));
        assert!(!is_github_host(
            "https://staticdelivery.nexusmods.com/a.png"
        ));
        assert!(!is_github_host("http://github.com@evil.com/x"));
    }

    #[test]
    fn version_compare() {
        assert!(is_newer("v1.2.0", "1.1.0"));
        assert!(is_newer("2.0.0.0", "1.9.9.9"));
        assert!(!is_newer("1.0.0.0", "1.0.0.0"));
        assert!(!is_newer("1.0", "1.0.0.1"));
        assert!(is_newer("v8.1.0.3", "8.1.0.2"));
    }
}
