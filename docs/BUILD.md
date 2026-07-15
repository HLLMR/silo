# Building Silo

## Prerequisites

- **Rust** (stable, MSVC toolchain on Windows), **Node 20+**, **npm**.
- **Tauri v2 system deps** per OS: https://v2.tauri.app/start/prerequisites/
  - Windows: **MSVC C++ build tools + Windows SDK** and WebView2 runtime.
  - macOS: Xcode command-line tools.
  - Linux: webkit2gtk + build essentials.
- Tauri CLI comes from `devDependencies` (`npm run tauri …`) — no global install.

## Commands

```bash
npm install            # frontend deps + Tauri CLI
npm run tauri:dev      # run the app (Vite + Rust, hot reload)
npm run tauri:build    # production bundle
# frontend only:
npm run build          # svelte-check + vite build  → dist/
# Rust only (from src-tauri/):
cargo test             # unit tests
cargo run --example scan_real            # scan the default mods folder, print summary
cargo run --example scan_real -- <path>  # scan a specific folder
```

## Windows toolchain

Requires a complete MSVC C++ toolset + Windows SDK (the "Desktop development with
C++" workload). With that installed, plain `cargo build` / `npm run tauri:dev`
work directly — cargo auto-detects MSVC via vswhere; no Developer-Command-Prompt
or `vcvarsall.bat` wrapper needed.

> History: an earlier broken VS install (a toolset with `cl.exe` but no `include/`
> `lib/`) caused `C1083: 'excpt.h' not found` on C deps like `vswhom-sys`. That's
> resolved — the toolchain is fixed and the workaround is no longer required. A cold
> clean build of the whole Tauri tree is ~57s.

## Verified so far

- Frontend builds: **~51 KB JS (~19 KB gzipped)**, 0 type errors.
- `cargo test`: parser + categorizer unit tests pass (6).
- `scan_real` on the real library: **727 mods, cold ~130–160 ms, warm ~5 ms**
  (SQLite cache), authoritative 2-level categories, DDS icons decode.
