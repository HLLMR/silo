fn main() {
    match silo_lib::github::check("Stephan-S", "FS25_AutoDrive", "1.0.0.0") {
        Ok(u) => println!("tag={} hasUpdate={} asset={:?} url={:?}", u.release.tag, u.has_update, u.release.asset_name, u.release.html_url),
        Err(e) => println!("ERR: {e}"),
    }
}
