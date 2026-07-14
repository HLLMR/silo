# Incumbent teardown — `MarkThor11/fs25-mt-mod-manager`

Distilled from a full read of the cloned source at `../fs25-mt-mod-manager/`
(Electron + React + Zustand + better-sqlite3). Purpose: mine the good ideas, and
turn every failure into a "do not repeat." Line refs are to that repo.

## Verdict

Right idea, wrong execution. The concept — a curated library projected into the
flat game folder via symlinks — is sound and worth adopting. But the app is
slow/clunky/buggy for **architectural** reasons that can't be spot-fixed, which is
why we're building fresh rather than PR-ing or forking. (Also: 27 commits over 5
days, "initial commit" is already the full monolith — no clean fork point exists.)

## Good ideas worth adopting

- **Symlink/junction-projected active set.** Source library in subfolders; mirror
  active mods into the flat game root so the engine sees them
  (`syncMirrorLinks`, `src/main/services/modManager.js:2107`). Hardlink fallback
  when symlink creation fails (`:2187`).
- **Per-savegame virtual folder.** `prepareVirtualModsFolder` builds a
  `VirtualActiveMods_SavegameN` folder of links for just that save's mods
  (`:253`), reading the save's mod list and heuristically detecting the map mod.
- **mtime+size scan cache.** `local_mod_cache` skips re-parsing unchanged mods
  (`:971-984`) — the one genuinely good perf decision.
- **Resumable downloads** table (`pending_downloads`) — reasonable idea for later.
- **Solid Electron security** — contextIsolation, sandbox, CSP, permission denial,
  clean namespaced preload. Match this posture (Tauri gets most of it by default).

## Do-not-repeat (performance)

- **Heavy work on the main thread.** `AdmZip` loads entire (100 MB+) zips into
  memory synchronously just to read `modDesc.xml`/icon (`:995`, `:1103`). The icon
  path decompresses *every* candidate image to check dimensions and runs a pure-JS
  `decodeDDS` pixel loop (`width×height`) inline (`:2619-2867`). → **Silo: Rust +
  rayon, stream archive central directory, DDS via a crate, cache decoded PNG.**
- **O(mods × DB-rows) index rebuild.** `getModHubMetadataPool` full-scans + JSON-
  parses the cache table, called once per weak-metadata mod in the scan loop and
  per dependency in the install loop (`cache.js:181`, called `modManager.js:1044`,
  `:3371`). → **Silo: normalized indexed tables, build once.**
- **Redundant full-tree work every scan.** `pruneBackups` (unbounded recursive
  walk), `syncMirrorLinks`, and `detectGamePath()` all fire on each `performScan`
  (`:889`, `:902`, `:1219`). Scan also double-fired at startup (visible twice
  16 ms apart in the committed `detection_log.txt`). → **Silo: one scan, debounced;
  separate cheap refresh from full rescan.**
- **Synchronous logging on the hot path.** `appendFileSync` per log line in two
  places (`modManager.js:12`, `cache.js:14`) produced a committed 11 MB /
  123k-line log. → **Silo: async, level-gated, never committed.**
- **Cache wiped on version bump.** A `v1.1.2` migration `DELETE FROM
  local_mod_cache` forces full re-extraction (`cache.js:134`). → **Silo: additive
  migrations; never discard the icon/meta cache.**
- **Base64 icons in DB rows + whole-list JSON over IPC** (`cache.js:370`, scan
  returns `iconData`/`storeData` per mod). → **Silo: PNG files on disk by ref.**
- **Build-time obfuscation** (`controlFlowFlattening` + `deadCodeInjection`,
  `vite.config.js`) — the user only runs the obfuscated build, several-fold
  slower. → **Silo: never obfuscate.**

## Do-not-repeat (renderer)

- **Whole-store Zustand subscriptions** on every card → any state change
  re-renders every card (`ModCard.jsx:54`, `InstalledModsPage.jsx:118`). Deep-scan
  replaces the whole `mods` array dozens of times (`useModHubStore.js:180-197`) →
  continuous flicker/reflow. → **Silo: selector subscriptions.**
- **No list virtualization** — 700+ live card subtrees, each with its own
  IntersectionObserver; `content-visibility` doesn't stop reconciliation. →
  **Silo: virtualize.**
- **`React.memo` defeated** by fresh inline objects/closures as props
  (`ModHubPage.jsx:1045`); heavy `filteredMods`/`folders` recompute every render
  (`InstalledModsPage.jsx:574`). → **Silo: stable props, memoized derivations,
  precomputed lookup Maps.**
- **Startup thundering herd** — 3 cache loads + category prefetch + deep-warming
  crawler + periodic stats + full scan, all during first paint (`App.jsx:37-85`).

## Do-not-repeat (correctness bugs observed)

- **Object-form `<dependency>` crashes the whole scan.** `dep.toLowerCase()` on an
  object `{title,url,modId}`, uncaught, rejects `scanLocalMods` → blank library
  (`modManager.js:1265`). → **Silo: typed dependency model; never assume string.**
- **Direct downloads can't be cancelled.** `'https'` install type never matched by
  `cancelInstall` (`:1823` vs `:2304`).
- **Fake update detection.** Fuzzy name-match on ModHub page 0 sets `hasUpdate` on
  any match; never compares versions — code admits it (`:2042`). A real
  `compareVersions` exists but isn't wired in. → **Silo: real version compare.**
- **Conflict detection is a stub.** Per-mod modDesc `<conflicts>` hardcoded empty
  (`:736 "// Empty conflicts array to fix ReferenceError"`); only duplicate
  script/specialization *names* are checked (`:1292`). → **Silo: this is our
  headline feature — do it properly.**
- **TDZ ReferenceError** on `hints` in an icon fallback, swallowed silently
  (`:3091`).

## Do-not-repeat (data backbone)

- **Scraping GIANTS' HTML as the backbone**, with cheerio class-name parsing, five
  download-URL fallback tiers, and a **hidden-BrowserWindow bot-evasion path**
  (spoofed `sec-ch-ua`, JS-clicking the download button) — `scraper.js:505`,
  `modManager.js:1561`. Fragile and ToS-gray. → **Silo: no scraper in v1;
  management works fully offline.**
- **Regex-based XML "parsing"** throughout `parseModDesc` — brittle vs attribute
  order/namespaces/CDATA. → **Silo: `quick-xml`.**

## Repo-hygiene lessons

Committed to their `main`: an 11 MB scan log, a 1 MB `debug_rgba.bin`, ~15
`test_*.js` scratch files, HTML page dumps, a crash log. → **Silo `.gitignore`
already blocks these classes; keep scratch out of the repo.**
