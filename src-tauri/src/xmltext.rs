//! One shared helper for pulling text out of a quick-xml `Text` event.
//!
//! quick-xml 0.41 split the old `BytesText::unescape()` — which decoded the bytes AND
//! resolved XML entities in one call — into a byte `decode()` plus a separate
//! `escape::unescape()` step. Our parsers relied on the one-call "give me the resolved
//! text" behaviour, so this restores it in a single place.

use quick_xml::events::BytesText;

/// Decode a text event to an owned `String` with XML entities resolved (`&amp;` → `&`),
/// or `None` if the bytes aren't valid text or an entity can't be resolved. Does not
/// trim — callers trim as needed.
pub fn text(t: &BytesText) -> Option<String> {
    let decoded = t.decode().ok()?;
    let unescaped = quick_xml::escape::unescape(&decoded).ok()?;
    Some(unescaped.into_owned())
}

#[cfg(test)]
mod tests {
    use super::text;
    use quick_xml::events::BytesText;

    #[test]
    fn resolves_entities_like_the_old_unescape() {
        // The whole reason this helper exists: a mod titled "Farms & Ranches" is stored as
        // `Farms &amp; Ranches` and must come back with the `&` — quick-xml 0.41 no longer
        // does that in `decode()`.
        let t = BytesText::from_escaped("Farms &amp; Ranches &lt;v2&gt;");
        assert_eq!(text(&t).as_deref(), Some("Farms & Ranches <v2>"));

        let plain = BytesText::from_escaped("no entities here");
        assert_eq!(text(&plain).as_deref(), Some("no entities here"));
    }
}
