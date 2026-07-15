fn main() {
    let rel = silo_lib::github::latest_release("Stephan-S", "FS25_AutoDrive", None).unwrap();
    println!("asset: {:?}", rel.asset_name);
    let dest = std::env::temp_dir().join("silo_dl_test.zip");
    let _ = std::fs::remove_file(&dest);
    match silo_lib::github::download_zip(rel.asset_url.as_deref().unwrap(), None, &dest) {
        Ok(()) => {
            let sz = std::fs::metadata(&dest).map(|m| m.len()).unwrap_or(0);
            println!("downloaded {} bytes, valid zip, at {:?}", sz, dest);
            let _ = std::fs::remove_file(&dest);
        }
        Err(e) => println!("ERR: {e}"),
    }
}
