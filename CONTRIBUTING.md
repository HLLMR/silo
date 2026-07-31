# Contributing to Silo

Thanks for taking the time to help. Silo is a native desktop **mod manager for
Farming Simulator 25** — a Tauri v2 app with a Rust core and a Svelte 5 frontend.
Issues and pull requests are welcome, whether it's a bug fix, a new panel, a
platform fix, or just a clearer error message.

By participating you agree to abide by our
[Code of Conduct](CODE_OF_CONDUCT.md).

## Dev setup

You need three things:

1. **Rust** (stable, via [rustup](https://rustup.rs)).
2. **Node 20+** ([nodejs.org](https://nodejs.org)) — CI builds on Node 20.
3. **Tauri v2 prerequisites** for your OS — the system libraries (WebView, build
   tools, etc.). Follow the official list:
   <https://v2.tauri.app/start/prerequisites/>. On Linux that's `webkit2gtk-4.1`
   and friends; see `.github/workflows/ci.yml` for the exact apt packages we
   install in CI.

Then, from the repo root:

```bash
npm ci                # install frontend deps from the lockfile
npm run tauri:dev     # run the app in dev (hot-reloads the frontend)
npm run tauri:build   # produce an installer in src-tauri/target/release/bundle/
```

The first `tauri:dev` compiles the Rust core, so expect a slow initial build;
subsequent runs are cached.

## Project layout

Silo is one app split across two halves:

- **`src-tauri/src/`** — the Rust core. All the real work lives here as small,
  pure modules behind thin `#[tauri::command]` wrappers: `scan`, `moddesc`,
  `category`, `icons`, `db`, `store`, `fsgame`, `organize`, `savegame`,
  `conflicts`, `xmlconfig`, `gamelaunch`, `github`, `siloapi` (catalog client),
  `logscan` (crash triage), `bisect` (guided bisection), `bindings` (input map),
  `mpsync` (multiplayer manifest), `bridge` (filltype companion generator), and
  more. Each module unit-tests without a running app.
- **`src/`** — the Svelte 5 (runes) + TypeScript + Vite frontend. Panels live in
  `src/lib/components/`. Design tokens are hand-authored CSS; no heavyweight UI
  kit. Long lists are virtualized.
- **`docs/`** — architecture and per-feature notes (`ARCHITECTURE.md`,
  `VISION.md`, `DESIGN.md`, `CROSS-PLATFORM.md`, and more). Read these before a
  large change.

## Working principles

These are the rules the codebase is built on (see [`CLAUDE.md`](CLAUDE.md) for the
full version). PRs are reviewed against them:

- **Off-thread by construction.** If it parses an archive, hashes, decodes an
  image, or walks a tree, it belongs in Rust on a worker thread — never in a
  blocking `invoke` handler, and never in the frontend. Keeping heavy work off
  the UI thread is the number-one lesson from the incumbent apps.
- **Every file write is reversible.** Never touch the user's game files
  destructively without an undo path. Prefer symlink/junction projection over
  moving or copying originals; back up before writing into
  `Documents/My Games/FarmingSimulator2025/`.
- **Parse, don't regex.** `modDesc.xml` and savegame XML go through `quick-xml`,
  not string scraping. Regex XML parsing is a top bug source in the incumbents.
- **Small, testable modules.** Keep the scan / library / projection / conflict
  engines as pure logic behind thin Tauri command wrappers, so they unit-test
  without launching the app.
- **Cross-platform, no hardcoded paths.** Windows, macOS, and Linux are all
  supported. Detect capability (e.g. symlink vs. junction vs. copy) and fall
  back gracefully rather than assuming a platform.
- **Cache aggressively, invalidate honestly** — keyed on path + mtime + size.
  Never wipe the whole cache on a version bump.

## Running the checks CI enforces

Before opening a PR, run what CI runs. Two workflows gate `main`
(`.github/workflows/ci.yml`):

**Frontend**

```bash
npm run check         # svelte-check — type/Svelte diagnostics
npm run build         # what CI actually runs: svelte-check + vite build
```

`npm run build` is the stricter gate — it runs `svelte-check` *and* the
production Vite build, catching type errors and a broken bundle in one step.

**Rust** (from `src-tauri/`)

```bash
cd src-tauri
cargo fmt --all --check   # CI fails on unformatted code — run `cargo fmt` to fix
cargo test                # the module unit tests (42+ and counting)
```

Please add or update tests when you change core logic — the modules are designed
to be tested without a running app, so there's no excuse not to.

## Commit messages

Match the existing history (`git log`). Keep the subject a short, imperative,
present-tense summary of the change, and scope it with a leading area where it
helps, e.g.:

```
Conflicts: detect cross-mod fillType overrides (SDK-verified true-positive)
fix(category): don't let mission-vehicle store items decide a mod's category
docs: bring CLAUDE.md/ARCHITECTURE current
CI: link libstdc++ on Linux for intel_tex_2's C++ ISPC code
```

Both a freeform `Area: summary` and a conventional-commit `type(scope): summary`
style appear in the log — either is fine. Keep it concise, explain the *why* in
the body if it isn't obvious, and reference an issue (`#123`) when there is one.

## Branch & PR flow

1. Fork the repo (or branch, if you have push access). Work off `main`.
2. Use a short descriptive branch name, e.g. `fix/duplicate-map-crash` or
   `feat/loadout-export`.
3. Make focused commits; keep unrelated changes out of the PR.
4. Run the checks above locally — they must pass.
5. Open a PR against `HLLMR/silo` `main`. Fill in the
   [pull request template](.github/pull_request_template.md): what and why, tests
   passing, off-thread/reversible where relevant, docs updated.
6. CI runs the frontend and Rust jobs on every PR; a maintainer reviews against
   the working principles above.

Small PRs get reviewed faster. If you're planning something large, open an issue
first so we can agree on the approach before you write it.

## Questions

Open an issue on [HLLMR/silo](https://github.com/HLLMR/silo/issues). For security
concerns, follow [SECURITY.md](SECURITY.md) instead of a public issue.
