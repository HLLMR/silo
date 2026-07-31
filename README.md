# Silo

**The mod manager Farming Simulator 25 should have shipped with.** A fast, native
desktop app that organizes your FS25 mod library, names the mod that crashed you,
catches conflicts before they bite, and pulls ModHub, GitHub and Nexus into one honest
catalog — without ever touching your saves destructively.

Free and open source. Windows · macOS · Linux. → **[silo.hllmr.com](https://silo.hllmr.com)**

> **Status: public beta.** [Download a build](https://github.com/HLLMR/silo/releases) or
> build from source (below).

## Why

FS25 reads one flat `mods/` folder, so everything you own is always active at once. When
something breaks, the game won't tell you what. There's no conflict detection, no
organization, no honest update tracking, no way to define a loadout per savegame. Silo
is the management layer the game left out.

## What it does

- **Crash & log triage** — parses `log.txt`, separates real errors from cosmetic noise,
  and names the mod at fault.
- **Conflict detection** — duplicate active maps (an instant crash), plus colliding
  filltypes, vehicle types and scripts across your active set.
- **Cross-source catalog** — one canonical record per mod aggregating ModHub + GitHub +
  Nexus, with the real latest version across all of them (backed by
  [SiloAPI](https://silo-api.hllmr.com)).
- **Guided bisection** — when the log can't name the culprit, automates "disable half,
  relaunch" to isolate it, and safely restores your active set afterward.
- **Loadouts & projection** — curate profiles and project only the active set into the
  game's flat folder at launch, via symlink/junction — never by moving your files.
- **Per-source actions** — star a repo on GitHub, endorse on Nexus, open the ModHub page
  to rate — through *your own* accounts. Silo brokers the action; it holds none of it.
- **Multiplayer sync**, **filltype-compatibility bridge generator**, **savegame backup**,
  and a **control-binding map**.

## Install

**Download:** grab the installer for your OS from
[Releases](https://github.com/HLLMR/silo/releases). Builds are unsigned (open source), so
Windows SmartScreen / macOS Gatekeeper may warn on first launch.

**Build from source:** needs [Rust](https://rustup.rs) + [Node 20+](https://nodejs.org)
and the [Tauri v2 prerequisites](https://v2.tauri.app/start/prerequisites/) for your OS.

```bash
npm ci
npm run tauri:dev     # run in dev
npm run tauri:build   # produce an installer in src-tauri/target/release/bundle/
```

## Trust

- **Open source** — every line is here; audit, fork, or build it yourself.
- **No telemetry, no account** — Silo phones home to nobody; there's nothing to sign up for.
- **Reversible by design** — it projects with links and backs up before it writes.

## Stack

Tauri v2 (Rust core, all heavy work off the UI thread) + Svelte 5 + TypeScript, SQLite
cache (path + mtime + size), quick-xml parsing. See [`docs/`](docs/) —
`ARCHITECTURE.md`, `VISION.md`, and per-feature notes.

## Contributing

Issues and PRs welcome. `npm run check` (svelte-check) and `cargo test` (in `src-tauri/`)
must pass — CI runs both. Please keep heavy work off the UI thread and file writes
reversible (see [`CLAUDE.md`](CLAUDE.md) for conventions).

## License

[MIT](LICENSE) © HLLMR. Use it, fork it, ship it.

---

_Not affiliated with GIANTS Software. Farming Simulator is a trademark of its respective
owner._
