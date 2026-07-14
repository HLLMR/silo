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

## Windows toolchain note (this dev machine)

The `C:\Program Files\Microsoft Visual Studio\18\Professional` install here is
**broken**: its MSVC toolset (`…\VC\Tools\MSVC\14.51.36231`) has `cl.exe` but **no
`include/` or `lib/` directories**, and `vcvarsall.bat` is missing — so C/C++
dependencies (e.g. `vswhom-sys`, pulled in via `tauri-build`) fail with
`fatal error C1083: Cannot open include file: 'excpt.h'`.

**Use the intact VS 2022 Build Tools instead.** Run cargo/tauri from a shell that
sourced its `vcvars64.bat`:

```
"C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
```

e.g. `cmd /c "<that path> && cargo run --example scan_real"`. With that environment
the whole Tauri tree compiles clean (~30s cold). To fix permanently, repair/install
the "Desktop development with C++" workload in the VS 18 installer, or set VS 2022
Build Tools as the default for Rust.

## Verified so far (v0.1 scan slice)

- Frontend builds: **47 KB JS (17.7 KB gzipped)**, 0 type errors.
- `cargo test`: modDesc parser unit tests pass.
- `scan_real` against the real library: **727 mods parsed in ~133 ms, 0 errors**.
