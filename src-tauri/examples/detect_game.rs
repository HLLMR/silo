fn main() {
    match silo_lib::gamelaunch::detect() {
        Some(g) => println!("FOUND: appId={} exe={} dir={}", g.app_id, g.exe, g.install_dir),
        None => println!("NOT FOUND"),
    }
}
