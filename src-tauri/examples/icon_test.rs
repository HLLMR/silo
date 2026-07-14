//! Verify icon extraction/decode on a real mod (no GUI).
//!   cargo run --example icon_test -- <mod.zip> [out.png]

use std::path::PathBuf;

fn main() {
    let path = PathBuf::from(
        std::env::args()
            .nth(1)
            .expect("usage: icon_test <mod.zip|mod_dir> [out.png]"),
    );
    let kind = if path.is_dir() { "dir" } else { "zip" };

    let xml = silo_lib::scan::read_moddesc_xml(&path, kind).expect("read modDesc.xml");
    let md = silo_lib::moddesc::parse(&xml);
    let icon = md.icon_filename.expect("mod has no <iconFilename>");
    eprintln!("iconFilename: {icon}");

    let png = silo_lib::icons::extract_icon_png(&path, kind, &icon)
        .expect("extract/decode icon");

    let out = PathBuf::from(std::env::args().nth(2).unwrap_or_else(|| "icon_out.png".into()));
    std::fs::write(&out, &png).expect("write png");
    println!("OK: decoded '{}' -> {} bytes PNG at {:?}", icon, png.len(), out);
}
