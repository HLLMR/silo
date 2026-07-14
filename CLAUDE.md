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
`VISION.md`, `MVP.md`, `ARCHITECTURE.md`, and `reference/`.

## Stack & structure (planned)

- **Tauri v2**: Rust core in `src-tauri/`, web frontend in `src/`.
- Rust does ALL heavy work (zip parsing, hashing, DDS/image decode, tree walks)
  on a thread pool — never block the UI. This is the #1 lesson from the incumbent.
- SQLite (rusqlite/sqlx) with real indexes; cache keyed by path+mtime+size.
- Frontend framework: TBD. Virtualized lists are mandatory, not optional.
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
  `docs/reference/incumbent-teardown.md`.
- FS25 SDK / game source: see workspace root `CLAUDE.md` and the memory index
  (`fs25-sdk-resource-locations`).
- Reference mod corpus: `Documents/My Games/FarmingSimulator2025/mods/` (700+
  real mods) — the scan/conflict engine's test fixtures.

## Per-change validation

1. Does any file-mutating path stay reversible? State how to undo it.
2. Did heavy work stay off the UI thread (Rust worker, not blocking invoke)?
3. Which real mod folder / savegame did you test against, and what was observed?
4. XML handled by a parser, not regex?
