fn main() {
    let dir = silo_lib::fsgame::user_dir().unwrap();
    println!("mods with settings: {:?}", silo_lib::settings_form::mods_with_settings(&dir));
    for p in silo_lib::settings_form::find_files(&dir, "pdlc_emergencyPack") {
        let f = silo_lib::settings_form::load_file(&p).unwrap();
        println!("\n{} ({} fields):", f.name, f.fields.len());
        for fld in &f.fields {
            println!("  [{}] {} = {} ({})", fld.id, fld.label, fld.value, fld.kind);
        }
        // dry-run edit id 0 -> verify apply produces valid change (not written to disk)
        let edited = silo_lib::settings_form::apply_edits(&f.raw, &[silo_lib::settings_form::Edit{id:0, value:"9".into()}]).unwrap();
        println!("  apply id0=9 -> contains integer=\"9\": {}", edited.contains("integer=\"9\""));
    }
}
