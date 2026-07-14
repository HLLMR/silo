# Silo — Architecture

## Principles

1. **The UI thread never does heavy work.** Archive parsing, hashing, image
   decode, and tree walks live in Rust on a worker pool. This is the single
   biggest lesson from the incumbent, whose sluggishness is 100% self-inflicted
   main-thread blocking.
2. **Library originals are sacred and untouched.** The game folder is a
   *projection* of a chosen profile, always reversible.
3. **Parse, don't regex.** XML via `quick-xml`.
4. **Cache honestly.** `path + mtime + size` invalidation; never nuke the whole
   cache on a version bump.

## High-level shape (Tauri v2)

```
┌─────────────────────────────────────────────┐
│  Frontend  (src/)  — web, virtualized UI     │
│  • library view, profiles, conflict panel    │
│  • talks to core via invoke + event streams  │
└───────────────▲───────────────┬──────────────┘
                │ events         │ invoke commands
                │ (progress,     │ (scan, build_profile,
                │  scan results) │  project, detect_conflicts…)
┌───────────────┴───────────────▼──────────────┐
│  Rust core  (src-tauri/)                      │
│                                               │
│  commands/        thin Tauri wrappers         │
│  scan/            walk + parse + hash (rayon)  │
│  moddesc/         quick-xml modDesc.xml model  │
│  library/         source library + tags/folders│
│  profile/         loadout model                │
│  projection/      symlink/junction/copy engine │
│  conflict/        static conflict analysis     │
│  savegame/        careerSavegame.xml reader     │
│  db/              rusqlite, migrations, cache   │
│  fsgame/          path discovery, launch        │
└───────────────────────────────────────────────┘
```

Keep `scan`, `moddesc`, `library`, `profile`, `projection`, and `conflict` as
**pure-ish logic modules** with their own unit tests, behind thin `commands/`
wrappers. They should be testable against a fixture folder without a running app.

## Key crates (proposed — confirm at scaffold time)

| Concern | Crate |
|---------|-------|
| Archive read | `zip` (read central directory without full extraction) |
| XML | `quick-xml` |
| Hashing | `blake3` (fast) or `md5` if we must match GIANTS `fileHash` |
| Parallelism | `rayon` for the scan fan-out; `tokio` for async IO if needed |
| Image decode | `image` + a DDS decoder crate (e.g. `ddsfile` / `image-dds`) |
| DB | `rusqlite` (bundled SQLite) with a small migration runner |
| Tree walk | `walkdir` |
| Windows links | `std::os::windows::fs` (symlink_dir/junction), `std::fs::hard_link` |

> Note on hashing: FS/GIANTS uses an MD5 `fileHash` in some places. For Silo's
> *own* change-detection, prefer `blake3`. Only compute MD5 if/when we need to
> match a GIANTS-visible value — decide per feature, don't default to MD5.

## Data model (SQLite — first pass)

Real tables with real indexes (contrast the incumbent's `LIKE 'mods_%'` KV scans):

- `mod_file(path PK, root, mtime, size, hash, kind)` — physical presence + cache key.
- `mod_meta(hash PK, mod_id, title, author, version, icon_ref, raw_moddesc_json)` —
  parsed metadata keyed by content hash so identical mods dedupe.
- `mod_dependency(hash, dep_ref)` / `mod_script(hash, script_basename)` /
  `mod_specialization(hash, spec_name)` — normalized, indexed for conflict queries.
- `folder(id, name, parent_id)` + `mod_tag(hash, tag)` — organization.
- `profile(id, name)` + `profile_mod(profile_id, hash, order_index)` — loadouts.
- `curation(hash, state, note)` — favorite/hidden/broken.
- Index every column we filter/join on (`hash`, `spec_name`, `script_basename`,
  `mod_id`, `LOWER(title)` via a generated/collated column — not runtime `LOWER()`).

Decoded icons cached as PNG files under the app data dir (referenced by
`icon_ref`), **not** as base64 blobs in SQLite rows (another incumbent mistake —
it bloats rows and the IPC payload).

## The projection engine (the delicate part)

At launch, project the active profile into the primary game `mods/` folder:

1. Determine link capability once: try creating a test symlink; if it fails, mark
   the root **copy-mode**. Dir junctions generally work without Dev Mode; file
   symlinks (for `.zip`s) may not — so `.zip` mods are the risky case.
2. For each active mod, ensure a link/copy in the game folder pointing at the
   library original; verify existing links point at the right target.
3. Remove stale projections that Silo created and are no longer active. **Only
   ever remove links/copies Silo owns** — track them; never delete user files we
   didn't create. (Tag Silo-created entries so cleanup is unambiguous.)
4. Everything reversible: "clear projection" restores the folder to empty/known
   state without touching the library.

Cross-volume note: hardlinks fail across drives and symlinks-to-another-drive may
be blocked — copy-mode is the universal fallback and must be first-class.

## Frontend

- Framework TBD (React or Svelte). Decide at scaffold; either is fine.
- **Virtualized lists mandatory** — the library view must render 700+ rows without
  materializing 700 DOM subtrees.
- Selector-based state subscriptions (no whole-store subscriptions → no re-render
  storms). Precompute lookups (status by fileName as a Map) once, not per row.
- Progress via Tauri events, not polling.

## Explicitly rejected (from incumbent teardown)

- Synchronous archive/image work anywhere near the UI thread.
- Regex XML parsing.
- Base64 icons in DB rows / whole-list JSON round-trips over IPC.
- Build-time obfuscation (`controlFlowFlattening`, `deadCodeInjection`).
- Scraping GIANTS' site with bot-evasion as the data backbone.
- Wiping the cache on version migrations.
