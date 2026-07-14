//! Mod icon extraction: pull a mod's `iconFilename` out of its archive/dir,
//! decode it (DDS BC1/BC3/BC7 or PNG/JPG), downscale to a thumbnail, and encode
//! PNG bytes. All CPU/IO — runs off the UI thread. Decoded thumbs get cached to
//! disk by the caller so this only runs once per mod version.

use image::imageops::FilterType;
use image::ImageFormat;
use std::hash::{Hash, Hasher};
use std::io::{Cursor, Read};
use std::path::Path;

/// Thumbnail edge length. Mod icons are typically 256² (store) / 512²; a 128²
/// thumb is crisp in the list and tiny to cache.
pub const THUMB: u32 = 128;

/// Return a cached PNG thumbnail as a `data:` URL, decoding + caching on first
/// use. Keyed by mod path + mtime + icon name, so it only decodes once per mod
/// version and survives app restarts. Returns None on any failure (missing icon,
/// undecodable format) so the UI just falls back to the placeholder tile.
pub fn cached_data_url(
    cache_dir: &Path,
    mod_path: &Path,
    kind: &str,
    icon_filename: &str,
) -> Option<String> {
    let png = cached_thumbnail(cache_dir, mod_path, kind, icon_filename).ok()?;
    use base64::Engine;
    let b64 = base64::engine::general_purpose::STANDARD.encode(&png);
    Some(format!("data:image/png;base64,{b64}"))
}

/// Disk-cached thumbnail PNG bytes.
pub fn cached_thumbnail(
    cache_dir: &Path,
    mod_path: &Path,
    kind: &str,
    icon_filename: &str,
) -> Result<Vec<u8>, String> {
    let meta = std::fs::metadata(mod_path).map_err(|e| e.to_string())?;
    let mtime = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_millis())
        .unwrap_or(0);

    let mut h = std::collections::hash_map::DefaultHasher::new();
    mod_path.hash(&mut h);
    mtime.hash(&mut h);
    icon_filename.hash(&mut h);
    let cached = cache_dir.join(format!("{:016x}.png", h.finish()));

    if let Ok(bytes) = std::fs::read(&cached) {
        return Ok(bytes);
    }
    let png = extract_icon_png(mod_path, kind, icon_filename)?;
    let _ = std::fs::create_dir_all(cache_dir);
    let _ = std::fs::write(&cached, &png);
    Ok(png)
}

/// Extract + decode + thumbnail a mod's icon, returning PNG bytes.
pub fn extract_icon_png(mod_path: &Path, kind: &str, icon_filename: &str) -> Result<Vec<u8>, String> {
    let raw = read_icon_bytes(mod_path, kind, icon_filename)?;
    let img = decode(&raw, icon_filename)?;
    let thumb = img.resize(THUMB, THUMB, FilterType::Triangle);
    let mut out = Cursor::new(Vec::new());
    thumb
        .write_to(&mut out, ImageFormat::Png)
        .map_err(|e| e.to_string())?;
    Ok(out.into_inner())
}

/// modDesc references the icon relative to the mod root; the extension it lists
/// doesn't always match the file on disk (e.g. `.dds` listed, `.png` shipped), so
/// try the swapped extension too.
fn icon_candidates(name: &str) -> Vec<String> {
    let name = name.replace('\\', "/");
    let mut v = vec![name.clone()];
    let lower = name.to_lowercase();
    if lower.ends_with(".dds") {
        v.push(format!("{}.png", &name[..name.len() - 4]));
    } else if lower.ends_with(".png") {
        v.push(format!("{}.dds", &name[..name.len() - 4]));
    }
    v
}

fn read_icon_bytes(mod_path: &Path, kind: &str, icon_filename: &str) -> Result<Vec<u8>, String> {
    match kind {
        "zip" => {
            let f = std::fs::File::open(mod_path).map_err(|e| e.to_string())?;
            let mut ar = zip::ZipArchive::new(f).map_err(|e| e.to_string())?;
            for cand in icon_candidates(icon_filename) {
                if let Ok(mut entry) = ar.by_name(&cand) {
                    let mut buf = Vec::new();
                    entry.read_to_end(&mut buf).map_err(|e| e.to_string())?;
                    return Ok(buf);
                }
            }
            Err(format!("icon '{icon_filename}' not found in archive"))
        }
        _ => {
            for cand in icon_candidates(icon_filename) {
                let p = mod_path.join(&cand);
                if p.is_file() {
                    return std::fs::read(&p).map_err(|e| e.to_string());
                }
            }
            Err(format!("icon '{icon_filename}' not found"))
        }
    }
}

fn decode(raw: &[u8], name: &str) -> Result<image::DynamicImage, String> {
    let is_dds = raw.starts_with(b"DDS ") || name.to_lowercase().ends_with(".dds");
    if is_dds {
        let dds = image_dds::ddsfile::Dds::read(Cursor::new(raw)).map_err(|e| e.to_string())?;
        let rgba = image_dds::image_from_dds(&dds, 0).map_err(|e| e.to_string())?;
        Ok(image::DynamicImage::ImageRgba8(rgba))
    } else {
        image::load_from_memory(raw).map_err(|e| e.to_string())
    }
}
