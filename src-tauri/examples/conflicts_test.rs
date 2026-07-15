//! Run conflict detection across the whole real library (as if every mod were
//! active) to surface real collisions and exercise the engine. No GUI, no writes.
//!
//!   cargo run --example conflicts_test

use silo_lib::conflicts::{self, ConflictInput};

fn main() {
    let root = silo_lib::fsgame::default_mods_paths()
        .into_iter()
        .next()
        .expect("no mods folder");
    let db_path = dirs::data_dir()
        .unwrap()
        .join("com.hllmr.silo")
        .join("silo.db");
    let conn = silo_lib::db::open(&db_path).expect("open db");
    let cache = silo_lib::db::load_cache(&conn);
    let out = silo_lib::scan::scan_cached(vec![root], &cache, |_, _| {});

    let inputs: Vec<ConflictInput> = out
        .result
        .mods
        .iter()
        .map(|m| ConflictInput {
            tech_name: m.tech_name.clone(),
            title: m.title.clone(),
            path: m.path.clone(),
            kind: m.kind.clone(),
        })
        .collect();

    eprintln!("checking conflicts across all {} mods…\n", inputs.len());
    let conflicts = conflicts::detect(&inputs);

    let crit = conflicts.iter().filter(|c| c.severity == "critical").count();
    let warn = conflicts.iter().filter(|c| c.severity == "warning").count();
    println!("=== conflicts if ALL {} mods were active ===", inputs.len());
    println!("critical: {crit}   warnings: {warn}\n");

    for c in conflicts.iter().take(25) {
        println!(
            "[{}] {} \"{}\"  ({} mods)",
            c.severity, c.kind, c.name, c.mods.len()
        );
        println!("   {}", c.mods.join(", "));
    }
    if conflicts.len() > 25 {
        println!("\n… and {} more", conflicts.len() - 25);
    }
}
