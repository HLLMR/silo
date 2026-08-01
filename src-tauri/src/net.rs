//! Outbound-URL validation — an SSRF guard for native HTTP. The webview and the catalog are
//! treated as potentially compromised, so any URL that reaches `ureq` from them is checked
//! first: HTTPS only (plain `http` allowed only to localhost, for a self-hosted / dev endpoint),
//! and never a loopback / link-local / private / unspecified IP host. Blocks a compromised
//! frontend or upstream from steering a native request at cloud metadata (169.254.169.254) or
//! an internal service.

use std::net::IpAddr;
use url::{Host, Url};

/// Validate a URL the native layer is about to fetch.
///
/// - Scheme must be `https`, except plain `http` to a loopback/localhost host when
///   `allow_localhost` (a self-hosted or dev endpoint).
/// - The host must not be a loopback / link-local / private / unspecified / CGNAT address
///   (unless it's loopback and `allow_localhost`).
///
/// Returns the parsed `Url` on success.
pub fn validate_outbound_url(raw: &str, allow_localhost: bool) -> Result<Url, String> {
    let url = Url::parse(raw).map_err(|_| format!("not a valid URL: {raw}"))?;
    let host = url.host().ok_or_else(|| "URL has no host".to_string())?;

    let is_loopback_host = match &host {
        Host::Domain(d) => d.eq_ignore_ascii_case("localhost"),
        Host::Ipv4(ip) => ip.is_loopback(),
        Host::Ipv6(ip) => ip.is_loopback(),
    };

    match url.scheme() {
        "https" => {}
        "http" if allow_localhost && is_loopback_host => {}
        _ => return Err("only https URLs are allowed".to_string()),
    }

    match host {
        Host::Ipv4(ip) => reject_bad_ip(IpAddr::V4(ip), allow_localhost)?,
        Host::Ipv6(ip) => reject_bad_ip(IpAddr::V6(ip), allow_localhost)?,
        Host::Domain(d) => {
            if d.eq_ignore_ascii_case("localhost") && !allow_localhost {
                return Err("refusing a localhost URL".to_string());
            }
        }
    }
    Ok(url)
}

fn reject_bad_ip(ip: IpAddr, allow_localhost: bool) -> Result<(), String> {
    if ip.is_loopback() {
        return if allow_localhost {
            Ok(())
        } else {
            Err("refusing a loopback address".to_string())
        };
    }
    let bad = match ip {
        IpAddr::V4(v4) => {
            v4.is_private()
                || v4.is_link_local()
                || v4.is_unspecified()
                || v4.is_broadcast()
                // 100.64.0.0/10 carrier-grade NAT
                || { let o = v4.octets(); o[0] == 100 && (o[1] & 0xc0) == 0x40 }
        }
        IpAddr::V6(v6) => {
            v6.is_unspecified() || {
                let s = v6.segments();
                // fc00::/7 unique-local, fe80::/10 link-local
                (s[0] & 0xfe00) == 0xfc00 || (s[0] & 0xffc0) == 0xfe80
            }
        }
    };
    if bad {
        Err("refusing a request to a private / link-local address".to_string())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_public_https_only() {
        assert!(validate_outbound_url("https://silo-api.hllmr.com/mods", false).is_ok());
        assert!(
            validate_outbound_url("https://github.com/x/y/releases/download/v1/a.zip", false)
                .is_ok()
        );
        assert!(
            validate_outbound_url("https://cdn27.giants-software.com/modHub/x.zip", false).is_ok()
        );
    }

    #[test]
    fn blocks_ssrf_and_non_https() {
        for bad in [
            "http://silo-api.hllmr.com/",             // not https
            "https://169.254.169.254/latest/meta",    // link-local (cloud metadata)
            "https://10.0.0.5/x",                     // private
            "https://192.168.1.1/x",                  // private
            "https://172.16.9.9/x",                   // private
            "https://100.64.0.1/x",                   // CGNAT
            "https://127.0.0.1/x",                    // loopback, no dev mode
            "https://[::1]/x",                        // loopback v6
            "https://[fc00::1]/x",                    // ULA v6
            "https://localhost/x",                    // localhost, no dev mode
            "ftp://example.com/x",                    // wrong scheme
            "not a url",
        ] {
            assert!(validate_outbound_url(bad, false).is_err(), "should block {bad}");
        }
    }

    #[test]
    fn allows_localhost_in_dev_mode_only() {
        assert!(validate_outbound_url("http://localhost:8787/mods", true).is_ok());
        assert!(validate_outbound_url("http://127.0.0.1:8787/mods", true).is_ok());
        assert!(validate_outbound_url("https://localhost:8787/mods", true).is_ok());
        // non-loopback private is still blocked even in dev mode
        assert!(validate_outbound_url("http://10.0.0.5/x", true).is_err());
        assert!(validate_outbound_url("https://192.168.0.2/x", true).is_err());
    }
}
