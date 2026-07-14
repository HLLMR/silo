# Silo

> Working title. A desktop app to organize, curate, and manage your Farming
> Simulator 25 mod library — the way a silo stores a harvest: clean, sealed,
> and retrievable, instead of one overflowing folder.

**Status:** planning / pre-scaffold. No application code yet — see [`docs/`](docs/).

## Why

FS25 has thousands of community mods and no real tooling to manage them. The game
reads one flat `mods/` folder, so everything you own is always "installed" and
active at once. There's no conflict detection, no organization hierarchy, no
honest update tracking, no way to define a loadout for a given save. Keeping a
large library usable currently costs hours that should be spent playing.

Silo is the management layer: a curated **source library** you organize freely,
projected into the game's flat folder only when you launch, with the conflicts,
updates, and per-savegame loadouts surfaced up front.

## What it is (and isn't)

- **Is:** a fast, native-feeling library manager — organization, profiles/loadouts,
  conflict detection, update tracking, savegame↔mod binding.
- **Isn't (for v1):** a ModHub browser/scraper. That's fragile, ToS-gray, and it's
  where the existing tools sink their effort while the management layer stays
  half-baked. We may add a browser later; the management layer is the moat.

## Stack

- **Core:** Tauri (Rust) — all heavy work (archive parsing, hashing, image decode,
  file-tree walks) runs on native threads, off the UI. ~10 MB binary.
- **UI:** web frontend (framework TBD), virtualized lists from day one.
- **Data:** SQLite (indexed), cached by path + mtime + size.
- **No build-time obfuscation.** Open, fast, small.

See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) and [`docs/MVP.md`](docs/MVP.md).

## Prior art we studied

- `MarkThor11/fs25-mt-mod-manager` (Electron) — good core idea (symlink-projected
  active set), poor execution. Teardown: [`docs/reference/incumbent-teardown.md`](docs/reference/incumbent-teardown.md).
- The Farm Sim Guy "game creator" tooling.

_Not affiliated with GIANTS Software._
