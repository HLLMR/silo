//! Dev harness: run the real scan pipeline against a mods folder and print a
//! summary — no GUI. Proves the core end-to-end and doubles as the seed of a CLI.
//!
//!   cargo run --example scan_real            # auto-detect default mods folder
//!   cargo run --example scan_real -- <path>  # scan a specific folder

use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let roots: Vec<PathBuf> = if args.is_empty() {
        silo_lib::fsgame::default_mods_paths()
    } else {
        args.iter().map(PathBuf::from).collect()
    };

    if roots.is_empty() {
        eprintln!("No mods folder detected. Pass a path: cargo run --example scan_real -- <path>");
        std::process::exit(1);
    }
    eprintln!("Scanning {} root(s): {:?}", roots.len(), roots);

    let result = silo_lib::scan::scan_with(roots, |done, total| {
        if total > 0 && done == total {
            eprintln!("  parsed {done}/{total}");
        }
    });

    let maps = result.mods.iter().filter(|m| m.is_map).count();
    let scripts = result.mods.iter().filter(|m| m.script_count > 0).count();
    let unique = result.mods.iter().filter(|m| m.unique_type.is_some()).count();
    let with_deps = result.mods.iter().filter(|m| !m.dependencies.is_empty()).count();
    let errors: Vec<_> = result.mods.iter().filter(|m| m.error.is_some()).collect();
    let digit: Vec<_> = result.mods.iter().filter(|m| m.ignored_digit_prefix).collect();

    println!("\n=== Silo scan ===");
    println!("mods:        {}", result.total);
    println!("took:        {} ms", result.took_ms);
    println!("maps:        {maps}");
    println!("script mods: {scripts}");
    println!("uniqueType:  {unique}");
    println!("with deps:   {with_deps}");
    println!("parse errors:{}", errors.len());
    println!("digit-prefix (game ignores): {}", digit.len());

    println!("\nfirst 8 by title:");
    for m in result.mods.iter().take(8) {
        println!(
            "  {:<40} v{:<10} {}{}",
            m.title.as_deref().unwrap_or(&m.tech_name),
            m.version.as_deref().unwrap_or("?"),
            if m.is_map { "[map] " } else { "" },
            m.author.as_deref().unwrap_or(""),
        );
    }

    if !errors.is_empty() {
        println!("\nparse errors (first 8):");
        for m in errors.iter().take(8) {
            println!("  {}: {}", m.tech_name, m.error.as_deref().unwrap_or(""));
        }
    }
    if !digit.is_empty() {
        println!("\ndigit-prefixed (silently ignored by the game):");
        for m in digit.iter().take(8) {
            println!("  {}", m.tech_name);
        }
    }
}
