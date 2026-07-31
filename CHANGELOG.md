# Changelog

All notable changes to Silo are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-07-30

First public beta.

### Added

- **Library scan & categorization** — scans the FS25 mod folder, reads each
  `modDesc.xml`, and categorizes mods (maps, vehicles, scripts, DLC, and more)
  with counts and scan timing. Warm scans are cached and fast (cache keyed by
  path + mtime + size).
- **Zip organizing** — moves loose mod zips into `mods/archive/<Category>/`;
  dev **folder** mods are left untouched (zip-only, never destructive).
- **Symlink/junction projection** — only the active set is projected into the
  game's flat `mods/` folder at launch, via symlink/junction with a copy
  fallback. Your original files are never moved.
- **Loadouts & profiles** — save the current active set as a loadout;
  apply / overwrite / delete; export to a `.silo` file and re-import. Active
  loadout name shows in the toolbar. Build a loadout from a savegame's mod list.
- **Curation** — favorite, hidden, and broken flags; star ratings; tags; and
  free-text notes, all persisted across rescans and usable as filters.
- **Detail drawer** — per-mod icon, metadata, and dependency status
  (present / missing), plus full-body descriptions and changelog, per-mod log
  health, and catalog/update status.
- **Conflict detection** — flags duplicate active maps (an instant-crash cause),
  plus colliding fill types, vehicle/unique types, and scripts across the active
  set, with severity and the mods involved.
- **Crash & log triage** — parses `log.txt`, separates real errors from cosmetic
  noise, and names the mod at fault with a plain-language verdict and ranked
  culprits.
- **Guided bisection** — when the log can't name a culprit, automates the
  "disable half, relaunch" search to isolate it; crash-safe, snapshotting and
  restoring the active set (including recovery after a mid-run force-quit).
- **Bindings map** — parses `inputBinding.xml` into a per-device control map and
  highlights inputs bound to more than one action (a view, not a verdict).
- **Multiplayer mod-set sync** — export a hashed manifest (`.silomp`) of the
  active set; a joiner verifies theirs against it with a fix-list
  (missing / wrong version / different file / extra).
- **Filltype-compatibility bridge generator** — generates a companion mod that
  adds a stubborn map fill type into the categories your equipment accepts (the
  "sugar beet" fix), with no vehicle edits. Output is per-user and reversible.
- **Savegame backup** — copies saves to a backup folder before edits.
- **Cross-source catalog (Browse)** — one canonical record per mod aggregating
  ModHub + GitHub + Nexus (backed by SiloAPI), with search, category filter,
  sort (popular / downloads / rating / newest / name), and pagination past the
  result cap. In-app GitHub install with a streaming progress bar; ModHub/Nexus
  are index + open-page.
- **Catalog-routed update checking** — checks the whole library against the
  catalog's real latest-across-sources, fixing the GitHub-vs-ModHub false
  "outdated" reports; per-mod update status also shown in the detail drawer.
- **Per-source interaction** — star/watch a repo on GitHub, endorse on Nexus, and
  deep-link to the ModHub page to rate — all through *your own* accounts. Silo
  brokers the action and holds none of your credentials.
- **Cross-platform** — Windows, macOS, and Linux, with per-OS projection and
  game-install discovery. Automatic mods-folder detection covers Windows and macOS;
  on Linux (Proton) the mods folder is set manually for now.

[Unreleased]: https://github.com/HLLMR/silo/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/HLLMR/silo/releases/tag/v0.1.0
