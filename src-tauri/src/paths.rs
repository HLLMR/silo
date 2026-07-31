//! Path-safety helpers for the Rust ↔ webview boundary. The webview is our own app, but
//! we treat it as potentially compromised (a hostile catalog string, a future bug), so
//! any file name or path that originated in the frontend/catalog is validated here before
//! it can drive a filesystem write. Cheap, and it turns "write anywhere" into "write only
//! where we intend".

use std::path::{Component, Path};

/// Accept a value only as a plain file name: exactly one normal path component, no
/// separators, no `.`/`..`, not absolute, no NUL. Rejects `../escape`, `a/b`, `C:\x`, etc.
/// so `root.join(name)` provably stays directly under `root`.
pub fn safe_file_name(name: &str) -> Result<&str, String> {
    let ok = !name.is_empty()
        && !name.contains(['/', '\\', '\0'])
        && name != "."
        && name != ".."
        && !Path::new(name).is_absolute()
        && Path::new(name).components().count() == 1
        && matches!(
            Path::new(name).components().next(),
            Some(Component::Normal(_))
        );
    if ok {
        Ok(name)
    } else {
        Err(format!("refusing an unsafe file name: {name:?}"))
    }
}

/// True when `target` (after resolving symlinks/`..`) lives inside `root`. Both must
/// exist to canonicalize; for a not-yet-created file, check its parent directory.
pub fn is_within(root: &Path, target: &Path) -> bool {
    match (root.canonicalize(), target.canonicalize()) {
        (Ok(r), Ok(t)) => t.starts_with(r),
        _ => false,
    }
}

/// Reject a path that contains a `..` component — a cheap guard against traversal on a
/// frontend-supplied path we're about to write to, without assuming a specific root
/// (so it doesn't break users with a custom mods folder).
pub fn no_traversal(path: &Path) -> Result<(), String> {
    if path.components().any(|c| matches!(c, Component::ParentDir)) {
        Err("refusing a path that contains '..'".to_string())
    } else {
        Ok(())
    }
}

/// Guard a destination we're about to WRITE: its parent directory must resolve inside one
/// of `allowed_roots`. Prevents a hostile path from redirecting a write outside the mod
/// folder(s). Returns Ok(()) when the parent is under an allowed root.
pub fn ensure_write_under(allowed_roots: &[std::path::PathBuf], dest: &Path) -> Result<(), String> {
    let parent = dest
        .parent()
        .ok_or_else(|| "destination has no parent directory".to_string())?;
    if allowed_roots.iter().any(|r| is_within(r, parent)) {
        Ok(())
    } else {
        Err("refusing to write outside the mods folder".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::safe_file_name;

    #[test]
    fn accepts_plain_names_rejects_traversal() {
        assert!(safe_file_name("FS25_Foo.zip").is_ok());
        assert!(safe_file_name("a mod (v1).zip").is_ok());
        for bad in [
            "",
            ".",
            "..",
            "../x",
            "a/b",
            "a\\b",
            "/etc/passwd",
            "C:\\Windows\\x",
            "sub/FS25_Foo.zip",
            "with\0nul",
        ] {
            assert!(safe_file_name(bad).is_err(), "should reject {bad:?}");
        }
    }
}
