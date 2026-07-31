# Silo — Claude project notes

Working title: **Silo**. A Tauri desktop app to manage the Farming Simulator 25
mod library. This is a standalone **application**, not an in-game mod — so it does
NOT follow the `FS25_<ModName>` / Lua-mod conventions in the workspace root
`CLAUDE.md`. Those apply to game mods; the shared "never invent GIANTS API" rule
is irrelevant here because Silo never runs inside the game engine.

## What this app is

The management layer FS25 lacks: organize a curated **source library**, define
**profiles/loadouts**, detect **conflicts**, track **updates**, and bind mods to
**savegames** — projecting the active set into the game's flat `mods/` folder only
at launch (via symlink/junction, with a copy fallback). See [docs/](docs/):
`VISION.md`, `MVP.md`, `ARCHITECTURE.md`, `SMOKE_TEST.md`, and `reference/`.

### Feature set (shipped as of 2026-07)

Well past the incumbents (FSG Mod Assistant, FarmSim Hub, MarkThor11 — see the
`fs25-mod-manager-decision` memory). Beyond the MVP management layer above:

- **Browse tab** — the SiloAPI canonical catalog (GitHub + ModHub + Nexus), search,
  category filter, pagination, per-source buttons w/ versions, in-app GitHub install
  with a streaming progress bar, detail drawer. ModHub/Nexus are index-only
  (open-page; their CDNs gate direct download).
- **Catalog-routed updates** — "⟳ Updates" checks the whole library against the
  catalog's latest-across-sources (fixes the GitHub-vs-ModHub false-"outdated" bug the
  incumbents have); per-mod update status also in the detail drawer.
- **Crash & log triage** (`◆ diagnose`) — parses `log.txt`, names the culprit mod,
  separates real errors from cosmetic noise.
- **Guided bisection** — automates "disable half, relaunch" to isolate a crash the log
  can't name; crash-safe (snapshots + restores the active set).
- **Conflict detection** — uniqueType/specialization/script collisions, **duplicate
  active map** (instant-crash), **cross-mod fillType override** (same-name last-wins;
  SDK-verified true-positive-only).
- **Bindings map** (`⌨ bindings`) — the full control map from `inputBinding.xml`,
  grouped by device, reused inputs highlighted (a view, not a verdict).
- **Multiplayer sync** — export a hashed manifest of the active set; a joiner verifies
  theirs against it (fix-list: missing / wrong version / different file / extra).
- **Filltype bridge** (`⛓ bridge`) — GENERATES a companion mod that adds a stubborn map
  filltype into the categories your equipment accepts (the "sugar beet" fix), no vehicle
  edits. Generator lives in Silo; output is per-user, no separate repo.

Backend: **SiloAPI** (separate repo `HLLMR/silo-api`, live at `https://silo-api.hllmr.com`).

## Stack & structure

Rust core modules (`src-tauri/src/`): `scan` `moddesc` `category` `icons` `db` `store`
`fsgame` `organize` `savegame` `conflicts` `settings_form` `xmlconfig` `gamelaunch`
`github` `siloapi` (catalog client) · `logscan` (crash triage) · `bisect` (guided
bisection) · `bindings` (input map) · `mpsync` (MP manifest) · `bridge` (filltype
companion generator). Each is pure logic behind thin `#[tauri::command]` wrappers with
unit tests (42+ Rust tests). Frontend panels in `src/lib/components/`.

- **Tauri v2**: Rust core in `src-tauri/`, web frontend in `src/`.
- **Frontend: Svelte 5 (runes) + Vite + TypeScript**, hand-authored CSS design
  tokens, no heavyweight UI kit. Virtualized lists mandatory. Design language in
  `docs/DESIGN.md`.
- Rust does ALL heavy work (zip parsing, hashing, DDS/image decode, tree walks)
  on a thread pool — never block the UI. This is the #1 lesson from the incumbent.
- SQLite (rusqlite/sqlx) with real indexes; cache keyed by path+mtime+size.
- **Cross-platform: Windows + macOS + Linux** (see `docs/CROSS-PLATFORM.md`).
  Per-OS game-file discovery and projection; never hardcode Windows paths.
- **No source obfuscation in builds.**

## Working principles

- **Never touch the user's game files destructively without a reversible path.**
  Prefer symlink/junction projection over moving/copying originals. Every
  write to `Documents/My Games/FarmingSimulator2025/` must be undoable.
- **Off-thread by construction.** If it parses an archive, hashes, decodes an
  image, or walks a tree, it lives in Rust on a worker — not in an `invoke`
  handler that blocks, and never in the frontend.
- **Parse, don't regex.** `modDesc.xml` and savegame XML go through `quick-xml`,
  not string scraping. (The incumbent's regex XML parsing is a top bug source.)
- **Windows-first, but symlink-safe.** File symlinks need Developer Mode/admin;
  dir junctions don't; hardlinks fail across volumes. Detect capability and fall
  back to copy-projection. This is the exact seam where the incumbent is "buggy."
- **Cache aggressively, invalidate honestly** (path+mtime+size). Never wipe the
  whole cache on version bumps.
- Small, testable Rust modules; keep the scan/library/projection/conflict
  engines as pure logic behind thin Tauri command wrappers so they unit-test
  without a running app.

## Reference material

- Incumbent source (for spec-mining, NOT copying):
  `../fs25-mt-mod-manager/` — Electron app. Teardown + bug list in
  `docs/internal/incumbent-teardown.md` (internal, git-ignored).
- FS25 SDK / game source: see workspace root `CLAUDE.md` and the memory index
  (`fs25-sdk-resource-locations`). Authoritative schemas live at
  `<game install>/shared/xml/schema/*.xsd` (88 of them; `modDesc.xsd`,
  `gameSettings.xsd`, `savegame_*.xsd` are the ones we use).
- **Local reference mirror:** `reference/` (gitignored) holds `schema/modDesc.xsd`,
  `schema/gameSettings.xsd`, and `samples/` — GIANTS' copyrighted files + the user's
  real save/mod samples, kept OUT of the public repo. Distilled facts are committed
  in `docs/reference/fs25-modding-notes.md`.
- Reference mod corpus: `Documents/My Games/FarmingSimulator2025/mods/` (**729**
  real mods on this machine) — the scan/conflict engine's test fixtures. NOTE:
  `modManagerTemplates/` and `modManagerArchives/` there are the incumbent's leftover
  folders — do not reuse those names.

## Per-change validation

1. Does any file-mutating path stay reversible? State how to undo it.
2. Did heavy work stay off the UI thread (Rust worker, not blocking invoke)?
3. Which real mod folder / savegame did you test against, and what was observed?
4. XML handled by a parser, not regex?
