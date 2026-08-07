# Changelog

All notable changes to Silo are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- **Browse caches each view, so going back to one is instant.** Revisiting a filter/sort combo
  you've already loaded no longer re-polls the catalog — results (and everything you'd paged in)
  come straight from an in-session cache.
- **Browse has a unified two-line filter bar.** The controls are now one organized bar: top
  line is **category (parent · subcategory) · search · sort · direction**, bottom line is the
  semantic facets (**region · era · theme · brand · realism**) · **year** · **Clear all**.
  Category is split into a parent dropdown and a subcategory dropdown (shown when the parent
  has children), and the header spans the full content width instead of stopping mid-page.

### Added

- **Silo now detects which mods have settings by looking inside them** — not just the ones
  that have already been run. It scans each mod for a shipped settings XML or Lua that
  persists to `modSettings/`, lifting detection from ~1 mod (only what had a runtime folder)
  to well over a hundred. Two new Library filters use it: **⚙ Has settings** and
  **⬆ Needs update** (join the ⚙/⟳ toggles next to Favorites / Flagged / In-conflict). The
  detail drawer badges a detected-but-not-yet-editable mod as "has settings — launch once
  to configure."

## [0.6.0] - 2026-08-06

### Fixed

- **Mod settings forms now recognize the `*Value` attribute convention** (`boolValue`, `intValue`,
  `floatValue`, `stringValue`) that real mods like Easy Dev Controls use — previously only
  `boolean`/`integer`/… were parsed, so those mods' settings came up as an empty form. Metadata
  attrs (e.g. `isSaved`) are preserved verbatim on save.

### Added

- **Filter Browse by facets.** A row of dropdowns in Browse scopes the catalog by **brand,
  theme, region, realism, and era** (from SiloAPI's semantic tags — pick one per facet and they
  combine), plus an **"available by year"** filter for period-correct playthroughs (only machines
  that existed by that year). A mod's tags also show in its detail drawer.
- **The library detail drawer now shows catalog info** — a mod's summary (clamped, with **Read
  more** for the full text), where it's **available** (ModHub / GitHub / Nexus, with links), and
  whether it's **outdated** (⬆ update available vs. ✓ up to date). No more hopping to Browse to
  find out if a library mod has a newer version.
- **An "updates" filter in the stat bar.** After running ⟳ Updates, a gold **N updates** chip
  appears when any library mods are outdated; click it to scope the list to just those. (Pairs
  with the drawer showing per-mod update status.)
- **The library auto-refreshes when you return to Silo.** Download a mod in your browser into
  the mods folder, alt-tab back, and it appears — no manual Rescan or restart. Triggered on window
  focus, throttled so quick tabbing doesn't re-scan repeatedly (the scan cache keeps it cheap).
- **Click a stat to filter the library.** The **maps**, **script mods**, **uniqueType**, and
  **active** counts in the stat bar are now toggle filters — click one to scope the list to it,
  click again (or another) to switch or clear. "active" gives you the quick "show only what's
  loaded" view.

## [0.5.0] - 2026-08-04

### Fixed

- **"Newest" in Browse now matches ModHub's order.** It was sorting by our catalog's
  refresh timestamp (so it barely changed anything); it now uses SiloAPI's `newest` sort,
  backed by ModHub's native grid ordinal — the same order you see on ModHub, updates and
  re-releases included. The web `/browse` "Newest" option uses it too.

### Added

- **Right-click menu on library mods.** Right-clicking a mod opens a context menu with its
  real commands — Activate/Park, Open details, **Find in Browse** (jumps to the catalog and
  searches for it), Change category, Edit settings, Favorite, Hide, Mark broken. The
  WebView's built-in reload/inspect menu is suppressed app-wide (cut/copy/paste still works
  in text fields).

### Changed

- **Collections is now its own toolbar entry**, split out of the Multiplayer panel.
  Sharing a curated set of mods as a link (and importing one) is a distinct feature from
  matching your active set to a host for a session, so it now lives under its own
  **Collections** button. `silo://collection` deep links open it directly.
- **A shared collection link is now a real page, not a raw gist.** Creating a collection
  gives you a `silo.hllmr.com/c/…` link. Recipients land on a page that shows the mod list
  and an **Open in Silo** button (with a download link if they don't have Silo yet) — instead
  of a wall of GitHub JSON. The gist/repo still backs it and gets a README pointing at the
  page. (Previously the "Open in Silo" button lived in the GitHub README, where GitHub's
  sanitizer strips `silo://` links, so it never worked.)
- **Manage your collections in-app.** The Collections panel now lists the collections you've
  published to your GitHub — copy a share link again, open its page, jump to it on GitHub,
  **Update** it (re-pin to your current active set, keeping the same share link), or delete it
  (secret gists delete in-app; public repos link out to GitHub, which Silo can't delete without
  extra permissions). No more digging through GitHub by hand.

## [0.4.0] - 2026-08-03

### Added

- **"Open in Silo" links for shared collections.** A public collection's page now has an
  **Open in Silo** button (a `silo://` link); clicking it opens the app straight to that
  collection's import — no copy-pasting the URL. Silo registers the `silo://` handler when you
  install it, and a second launch focuses the running window instead of opening a duplicate. (#72)

- **Release artifacts now carry a GitHub build-provenance attestation.** You can verify any
  download came from Silo's public build pipeline — `gh attestation verify <file> --repo HLLMR/silo`.
  See [docs/VERIFY_DOWNLOAD.md](docs/VERIFY_DOWNLOAD.md) for verifying the signature, checksum, and
  provenance of a download. (#60)

- **Every release includes a Software Bill of Materials** (`silo-sbom.cdx.json`, CycloneDX) — a
  full inventory of the Rust crates and npm packages (with licenses) that make up the build, so
  anyone can audit exactly what's inside. (#60)

## [0.3.1] - 2026-08-03

### Added

- **Windows installers are now code-signed** (Azure Trusted Signing). The `.exe`/`.msi` carry a
  verified publisher, so Windows SmartScreen no longer flags them as from an "unknown publisher."
  macOS/Linux builds remain OS-unsigned for now. (#60)

- **Dev mods are marked with a "Dev" badge in the library.** Unpacked folder mods (a development
  build you're working on, not a packaged `.zip`) now carry a badge so they're easy to spot and
  clearly distinct from managed library mods. Pairs with the earlier fix that made symlinked dev
  mods show up at all. (#76)

### Fixed

- **Release downloads now include a `SHA256SUMS.txt`** so you can verify your installer. The job
  that generates it used the wrong GitHub API endpoint and had silently never produced the file;
  it now attaches on each release. (#82)

## [0.3.0] - 2026-08-02

### Added

- **Collections — share a mod set as a link.** From Multiplayer sync, "Share as a link" publishes
  your set to your own GitHub — a **secret gist** for private/group sharing, or a **public,
  forkable repo** (with a generated README) for sharing widely — and hands you a URL (no mod files
  are uploaded, only the list — each mod pinned to a version and a content hash). Anyone you send it
  to can paste that link — gist *or* `github.com/owner/repo` — into "Open a shared link" to preview
  exactly what they already have, what's a different version,
  what Silo can install for them, and what they'll need to grab from ModHub/Nexus — plus a heads-up
  for any dependency gaps or mod conflicts among the mods they already have — then import it:
  Silo downloads the installable mods, verifies each against the shared build (verified / modified),
  and saves the whole set as a loadout to apply. Sharing needs a one-time "Enable collection sharing"
  (the GitHub `gist` permission) in Settings; a secret gist is unlisted, not password-protected.
  (#61)

- **Library health flags foreign files in your mods folder.** If a file sits in the flat `mods/`
  root at a name Silo manages but isn't the projection Silo created (a build you swapped in, a
  leftover from another tool), Library health now surfaces it — so you know the mod that loads
  from that name may not be the one you expect. Silo still never touches it. Detected by
  identity (hardlink / symlink / marker / content), reusing the same check that guards deletions.
  (#34)

### Fixed

- **Activating an archived mod now works even if Silo's database drifted from disk.** If a mod
  sat in `mods/archive/` but wasn't in Silo's internal manifest (e.g. after the database was
  reset while the archive folder persisted), selecting it as active did nothing and it never
  loaded — with no error shown. Silo now reconciles the manifest from what's physically in
  `archive/` on every scan, so every archived mod is activatable. (#75)

- **Dev mods symlinked into your mods folder now show up.** A mod added as a symlink — a common
  developer setup, e.g. linking a mod's source folder into `mods/` — was skipped by the library
  scan, because a symlink's own file type is neither a plain file nor a folder. Silo now follows
  the link to see what it points at, so symlinked dev mods (and Silo's own folder-mod projections)
  are scanned like any other. (#76)

- **A savegame created while playing now appears on Rescan** instead of only after restarting
  Silo. The savegame list was read once at startup; Rescan now refreshes it alongside the mod
  library. (#77)

## [0.2.4] - 2026-08-01

### Security

- **Direct GitHub updates now confirm the download is the right mod before replacing it.** A GitHub
  release can hold many assets (source archives, docs, other editions), and a valid ZIP of the
  wrong mod is still a valid ZIP. Before overwriting an installed mod, Silo now requires the
  downloaded archive to contain a readable root `modDesc.xml` and — when the current file is a
  readable mod — to identify as the *same* mod (matching `uniqueType`, or author + title). A
  provable mismatch is refused, with the original untouched; missing/uncomparable fields never
  block a legitimate update. Mirrors the identity check catalog installs already do. (#20)

## [0.2.3] - 2026-08-01

### Security

- **Warn when credential storage isn't encrypted.** On a machine with no OS keychain, connecting
  a GitHub/Nexus account falls back to storing the token in the local database. Settings now
  probes keychain availability and shows a clear warning above the account sections so it's never
  a silent fallback — the user decides before connecting. (#20)

- **SSRF guard on native requests.** A custom SiloAPI base URL and every mod download are now
  validated before the native layer fetches them: HTTPS only (plain `http` allowed only to
  localhost, for a self-hosted/dev endpoint), and never a loopback/link-local/private/CGNAT
  address. Blocks a compromised webview or catalog from steering a request at cloud metadata
  (169.254.169.254) or an internal service. Catalog images were already host-allowlisted. (#20)

- **Hardened the native command boundary against a compromised webview.** File-path commands
  that took a raw path from the frontend now validate it: loadout/mod-set/report export and
  import enforce the expected file type and reject `..` traversal, and config/mod-settings reads
  are confined to the FS25 user directory (mirroring the existing guarded writes). Turns
  "read/write any path" into "read/write an expected file type in an intended location." No
  behavior change for normal dialog-driven use. (#20)

## [0.2.2] - 2026-08-01

### Added

- **In-app auto-update.** Silo checks GitHub Releases on launch for a newer signed build and,
  when one is available, shows an **Update to vX** button in the top bar — one click downloads,
  verifies the signature, installs, and relaunches. Update artifacts are cryptographically
  signed in CI (minisign); the check fails closed and silently if there's no release, no
  network, or you're on a dev build. Auto-update takes effect from this build forward — install
  0.2.2 once, and future updates are one click.

## [0.2.1] - 2026-08-01

### Added

- **Version shown in the UI.** The app version now appears beside the **Silo** wordmark in the
  top bar and in a footer line at the bottom of Settings, read from the app's package info so it
  always matches the build. Makes "what version are you on?" answerable at a glance.

### Fixed

- **Doubled "v" in version labels.** Catalog sources that already prefix their version with a
  "v" produced `vv1.2.3` on Browse cards (and could on library rows). The label now strips any
  leading "v" before adding its own, so it always reads `v1.2.3`.
- **GitHub "Enable actions" now shows the device code.** Escalating an already-connected,
  read-only account to Star/Watch scope opened GitHub's device page but never surfaced the
  code in Silo — the code display was unreachable while connected. It now renders during the
  read-only → actions escalation.

## [0.2.0] - 2026-08-01

### Added

- **Mod integrity verification (provenance).** The detail drawer can check an installed mod
  against the trusted build SiloAPI hashed from its source: **Verified** (byte-for-byte or
  content match), **Modified** (with the exact list of changed / added / removed files — the
  injected-code candidates), or **Unverified** (no hashed build to compare against yet). The
  verdict is remembered as you move around, and verified/modified mods carry a badge in the
  library list. Byte-exact against the server via a cross-language-ratified manifest format.
  It's a provenance check, not a virus scan — it confirms origin and integrity, and says so.
  Coverage spans GitHub and ModHub sources, popular-first, and grows every hour.
- **Organize preview.** The manual **Organize** button now shows a read-only dry run
  first — exactly which zips move into which `archive/<Category>/` folder — before a
  single file is touched. Confirm to apply, Cancel to touch nothing. Dismissible for
  good with "Don't preview next time" so power users aren't nagged. (The opt-in
  auto-filer still runs silently — you already chose it.)

### Security & reliability (hardening pass)

- **Config & mod-settings edits are confined and atomic.** Editing a game config or a mod's
  settings now writes only inside the FS25 user folder, always succeeds a backup first (it
  won't overwrite if it can't back up), and swaps the new file in atomically — so a
  disk/permission failure can never leave a half-written or unbacked config.
- **Silo only removes files it can prove it created.** Before de-activating or flattening,
  Silo now verifies the file in the mods folder really is its own projection (the hardlink,
  the junction/symlink, or a marked copy). If you swapped in your own build at that name, it
  is left untouched **and now says so** instead of silently doing nothing — closing the last
  realistic data-loss path (a user replacement being removed because the filename matched).
- **Migrated tokens leave no residue.** When a token from an older build is moved from the
  database into the OS keychain, Silo now compacts the database afterward so the old
  plaintext bytes can't be carved back out of it.
- **The backend owns where it writes.** Installs, organizing, and updates now validate the
  target mods folder against the game folder(s) Silo detected, rather than trusting a path
  handed in by the UI — a defence-in-depth boundary on the Rust side.
- **Direct catalog installs are validated like updates — and identity-checked.** A downloaded
  mod must be a fully openable `.zip` containing `modDesc.xml` before it's admitted, so a
  truncated or garbage download can't land as a "broken mod". And when the catalog has a
  canonical hash for the mod (GitHub-source), the downloaded bytes must match it byte-for-byte
  — a valid ZIP of the *wrong* mod, or a tampered asset, is refused.
- **Safer cross-volume moves.** A move that can't be a fast rename now copies to a temporary
  file and swaps it into place, so an interrupted move never leaves a half-written file.
- **Organize can never delete your only copy.** The restore/deactivate paths now
  refuse to remove a file from the mods root unless the archived copy exists behind
  it — so an update or organize interrupted by a crash or power loss leaves your
  original untouched rather than risking the last copy. Added a regression test for
  that exact interrupted state.
- **Mod downloads carry no credentials.** Release assets are public and are now
  fetched unauthenticated; your GitHub token is used only for explicit actions you
  take (star / watch), never attached to a download, and never over plaintext HTTP.
- **Update writes are confined to your mods folder.** The destination is validated to
  resolve inside a real mods root before anything is written.
- **Interrupted updates roll back.** If an in-place update fails mid-write, Silo
  restores the previous version from its backup instead of leaving a truncated file.
- **Hardened XML parsing.** Upgraded `quick-xml` to close two denial-of-service
  advisories (RUSTSEC-2026-0194 / 0195) a malformed mod could otherwise trigger.
- **The CI dependency audit is now a release gate** — the build fails on any known
  vulnerable dependency.

### Changed

- Catalog images now load from SiloAPI's server-side cache when available (faster and
  referer-free), falling back to the in-app proxy for not-yet-cached images.

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
  game's flat `mods/` folder at launch, via symlink/junction (zips via hardlink)
  with a copy fallback. Organizing moves your zips into a local `archive/` folder,
  fully reversible with one-click Flatten; nothing ever leaves your machine.
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
