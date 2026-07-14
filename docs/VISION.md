# Silo — Vision & Positioning

## The problem

Farming Simulator 25 has thousands of community mods and effectively **no tooling
to manage them**. The concrete pain:

- The game reads **one flat `mods/` folder**. Everything you own is always active.
  There is no notion of "installed but disabled," no loadouts, no per-save sets.
- **No conflict detection.** Two mods that patch the same script or register the
  same specialization silently break your save — you find out in-game, hours later.
- **No organization hierarchy.** One giant folder of `.zip`s with cryptic names.
- **No honest update tracking.** You don't know what's out of date, removed from
  ModHub, or superseded.
- **Bad/unfinished mods** accumulate with no curation trail.

The net effect: keeping a large library usable costs **hours that should be spent
playing.** That's the wedge.

## The insight

The valuable layer is **management, not discovery.** A ModHub browser is:

- **Fragile** — it depends on scraping GIANTS' HTML, which changes.
- **ToS-gray** — the incumbent already ships bot-evasion (spoofed headers, hidden
  browser windows clicking download buttons) to get past detection.
- **Replaceable** — GIANTS could add first-party browsing and erase that feature
  overnight.

Management is the opposite: sticky, defensible, and something GIANTS has shown no
interest in building. So Silo leads with management and treats discovery as an
optional, later add-on.

## The mental model

```
   SOURCE LIBRARY  (Silo owns this — organized, tagged, curated)
        │
        │  project the ACTIVE SET at launch
        ▼
   GAME mods/ FOLDER  (flat, what FS25 actually reads)
```

The user organizes freely in the library. Silo projects a chosen **profile /
loadout** into the game's flat folder only when they launch — via symlink/junction
(copy fallback where symlinks aren't permitted). The game sees exactly the set you
chose; your library stays clean and reversible.

## Positioning

Aspiration: the **Nexus-Mod-Manager-caliber** tool for FS25 — a real application,
not a hobby script. But the FS community is smaller and the game's needs are
specific (maps are huge, savegames bind mod lists, the folder is flat), so we fit
the tool to *this* game rather than porting a generic manager.

**Our competitive wedge is the incumbents' headline weakness: speed and stability.**
Both existing tools are new and neither nails the management layer; the most-used
one is slow, clunky, and buggy specifically because it does heavy work on a JS UI
thread. A Tauri app that scans a 700-mod library in a snap and never corrupts the
folder is a direct, felt differentiator — "native, fast, tiny."

## Competitors

| Tool | Notes |
|------|-------|
| `MarkThor11/fs25-mt-mod-manager` | Electron. Good core idea (symlink-projected active set, per-savegame virtual folder), poor execution (blocks main thread, no virtualization, scraping-first, self-obfuscated). Teardown in `reference/incumbent-teardown.md`. |
| Farm Sim Guy "game creator" | Very new; different emphasis. Evaluate for feature gaps. |

## Non-goals (for now)

- Not a ModHub scraper/browser in v1.
- Not a mod *editor* (that's GIANTS Editor/Studio territory).
- Not multiplayer/server-config management (possible later).
- Not cross-game (FS25-specific until proven).

## North-star outcomes

1. Opening Silo with a 700-mod library is instant and never freezes.
2. You can define "Chill Seasonal Playthrough" vs "Heavy Machinery Test" loadouts
   and switch between them in one click before launch.
3. Silo tells you *before* you launch that mod A and mod B will conflict.
4. You always know what's out of date, and updating is one click and reversible.
