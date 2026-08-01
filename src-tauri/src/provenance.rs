//! Provenance manifest hashing — the client half of the SiloAPI provenance contract.
//!
//! See `docs/PROVENANCE.md` (client) and SiloAPI `docs/PROVENANCE-CANONICALIZATION.md`
//! (the ratified byte format). The whole tamper-verification feature turns on this hash
//! matching the server's **byte-for-byte** — a one-character format divergence would make
//! every mod read "modified" (the exact false-positive the contract exists to prevent). So
//! the pure `manifest_hash` computation lives here behind the §7 conformance vectors, and
//! is ratified (green vectors on both sides) BEFORE any zip/scan integration is built.
//!
//! Ratified decisions (answers to the spec's §10 open items):
//!   §10.1 sort  — by the UTF-8 byte sequence of the path.
//!   §10.2 NFC   — yes, NFC-normalize the decoded path (fewer false "modified" verdicts).
//!   §10.3 decode— UTF-8-flag-aware with CP437 fallback happens at the zip-read layer
//!                 (upstream of this pure function); CP437 is a static table, so parity-safe.
//!   §10.4 hash  — per-entry sha256 is over UNCOMPRESSED bytes (computed upstream).
//!   §10.5 vectors — reproduced in the tests below.

use sha2::{Digest, Sha256};
use unicode_normalization::UnicodeNormalization;

/// One regular-file entry: its canonicalized path and the lowercase-hex sha256 of the
/// entry's UNCOMPRESSED bytes.
#[derive(Debug, Clone)]
pub struct Entry {
    pub path: String,
    pub sha256: String,
}

fn hex_lower(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        use std::fmt::Write;
        let _ = write!(s, "{b:02x}");
    }
    s
}

/// Canonicalize a raw zip entry name per spec §3: `\`→`/`, strip a single leading `./`
/// and any leading `/`, NFC-normalize, case preserved. (Decode and directory exclusion
/// happen at the zip-reading layer, upstream of this.)
pub fn canon_path(raw: &str) -> String {
    let slashed = raw.replace('\\', "/");
    let trimmed = slashed
        .strip_prefix("./")
        .unwrap_or(&slashed)
        .trim_start_matches('/');
    trimmed.nfc().collect()
}

/// Compute `manifestHash` from the file entries (spec §4): canonicalize paths, sort by
/// UTF-8 byte order, emit `<path>\0<sha256-hex>\n` per entry, sha256 the concatenation.
/// Returns lowercase hex. Order-independent of the input (it sorts).
///
/// Duplicate canonical paths are a spec anomaly (§4) — the caller that builds `Entry`s
/// from a real zip must fail loud on them; this pure function assumes distinct paths.
pub fn manifest_hash(entries: &[Entry]) -> String {
    let mut norm: Vec<(String, &str)> = entries
        .iter()
        .map(|e| (canon_path(&e.path), e.sha256.as_str()))
        .collect();
    // §4.2 — UTF-8 byte order. Rust's default `String` Ord already IS byte order; sorting
    // on `.as_bytes()` makes the contract explicit and immune to any future Ord change.
    norm.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
    let mut h = Sha256::new();
    for (path, sha) in &norm {
        h.update(path.as_bytes());
        h.update([0x00]);
        h.update(sha.as_bytes());
        h.update([0x0a]);
    }
    hex_lower(&h.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn e(path: &str, sha: &str) -> Entry {
        Entry {
            path: path.into(),
            sha256: sha.into(),
        }
    }

    // ── SiloAPI docs/PROVENANCE-CANONICALIZATION.md §7 — ratification vectors ──
    // Green here == the byte format is agreed with infra == infra builds the pipeline.

    #[test]
    fn vector_1_ascii() {
        let entries = [
            e(
                "modDesc.xml",
                "45fcb867bb43ef043f1f1fe5ea6f95edd2099a71152259d35358a03907704c2c",
            ),
            e(
                "scripts/main.lua",
                "d287bb7f9d15abdc5b6e98536263815744b6ef21c8f3c839fc434ca70d8efe99",
            ),
            e(
                "maps/map01.i3d",
                "441f2166bc95eb490f49e712ac4cfd2f1dfc1a57f5111b699965eadf47c6dd0c",
            ),
        ];
        assert_eq!(
            manifest_hash(&entries),
            "9fea10cd5565c39b36fe3c6d977f92b950fbbcfe1774849143340ba5dbcb8792"
        );
    }

    #[test]
    fn vector_2_unicode_nfc() {
        // `ü` = U+00FC (NFC, precomposed) — written as an explicit escape so the source
        // file's own encoding can't smuggle in a decomposed form.
        let entries = [
            e(
                "modDesc.xml",
                "ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb",
            ),
            e(
                "scripts/Z\u{00FC}rich.lua",
                "594e519ae499312b29433b7dd8a97ff068defcba9755b6d5d00e84c524d67b06",
            ),
            e(
                "scripts/a.lua",
                "3e23e8160039594a33894f6564e1b1348bbd7a0088d42c4acb73eeaed59c009d",
            ),
        ];
        assert_eq!(
            manifest_hash(&entries),
            "8dceff0991ea088ca8a27b774396122fdf54d38c3aba91c47b303e422e19433e"
        );
    }

    #[test]
    fn empty_manifest_is_sha256_of_nothing() {
        assert_eq!(
            manifest_hash(&[]),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn canon_normalizes_separators_and_prefixes() {
        assert_eq!(canon_path("./scripts\\main.lua"), "scripts/main.lua");
        assert_eq!(canon_path("/a/b.lua"), "a/b.lua");
        // NFC: a decomposed `u`+combining-diaeresis collapses to U+00FC.
        assert_eq!(canon_path("u\u{0308}.lua"), "\u{00FC}.lua");
    }

    #[test]
    fn hash_is_input_order_independent() {
        let a = [
            e(
                "b.lua",
                "3e23e8160039594a33894f6564e1b1348bbd7a0088d42c4acb73eeaed59c009d",
            ),
            e(
                "a.lua",
                "45fcb867bb43ef043f1f1fe5ea6f95edd2099a71152259d35358a03907704c2c",
            ),
        ];
        let b = [a[1].clone(), a[0].clone()];
        assert_eq!(manifest_hash(&a), manifest_hash(&b));
    }
}
