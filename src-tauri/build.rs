fn main() {
    // `image_dds` pulls in `intel_tex_2`, whose ISPC texture-compression code is C++ and
    // references the C++ runtime (e.g. `__gxx_personality_v0`). MSVC/macOS link the C++
    // stdlib implicitly, but on Linux with lld it must be requested explicitly or the
    // final link fails with an undefined symbol. Gated on the TARGET os (build scripts
    // get CARGO_CFG_TARGET_OS), so it's cross-compile-safe.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
    tauri_build::build();
}
