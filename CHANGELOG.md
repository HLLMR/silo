# Changelog

All notable changes to Silo are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **In-app auto-update.** Silo checks GitHub Releases on launch for a newer signed build and,
  when one is available, shows an **Update to vX** button in the top bar — one click downloads,
  verifies the signature, installs, and relaunches. Update artifacts are cryptographically
  signed in CI (minisign); the check fails closed and silently if there's no release, no
  network, or you're on a dev build. Auto-update takes effect from this build forward.

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
