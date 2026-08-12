# Silo v0.1.0 — first public beta

**Silo is the mod manager Farming Simulator 25 should have shipped with** — a
fast, native desktop app that organizes your FS25 mod library, tells you which
mod crashed you, catches conflicts before they bite, and pulls ModHub and GitHub
into one honest catalog — without ever touching your saves
destructively. Free, open source, Windows · macOS · Linux.

## Highlights

- **Know what crashed you.** Silo reads `log.txt`, filters cosmetic noise from
  real errors, and names the culprit mod in plain language. When the log can't
  say, guided bisection isolates it for you — and restores your active set
  exactly when it's done.
- **Catch conflicts before they bite.** It flags duplicate active maps (an
  instant crash), plus colliding fill types, vehicle types, and scripts across
  your active set — with severity and the mods involved.
- **One honest catalog across every source.** Browse a single canonical record
  per mod aggregating ModHub + GitHub, with the real latest version
  across all of them. Search, filter, and sort by popularity, downloads, or
  rating — no more false "outdated" flags from mismatched sources.
- **Act through your own accounts.** Star or watch a repo on GitHub, or deep-link
  to ModHub to rate — Silo brokers the action and keeps none
  of your credentials.
- **Loadouts, projected safely.** Curate profiles and project only the active set
  into the game's flat `mods/` folder at launch via symlink/junction — never by
  moving your files.
- **Built to be trusted.** Fully open source, no telemetry, no account, and
  reversible by design — it projects with links and backs up before it writes.

Also in the box: a control-binding map, multiplayer mod-set sync, a
filltype-compatibility bridge generator, and savegame backup.

## Install & first run

1. Download the installer for your OS from the assets below.
2. Builds are **unsigned** (this is open source), so **Windows SmartScreen** or
   **macOS Gatekeeper** may warn on first launch — this is expected. On Windows,
   choose "More info → Run anyway"; on macOS, open from the right-click menu.
3. On first run, point Silo at your FS25 `mods` folder and let it scan.

## Beta — known limits

- **Auto-update isn't in yet** — grab new releases from this page for now.
- **ModHub is index + deep-link** — Silo shows its version and opens the page
  (its CDN gates direct downloads). In-app install works for GitHub-hosted mods.
- **Catalog coverage is still filling in** — the cross-source catalog is growing;
  some mods may not be indexed yet.

Found a rough edge? Please file it — beta feedback is exactly what shapes v0.1.x.

## Links

- Website: https://silo.hllmr.com
- Source: https://github.com/HLLMR/silo
- Issues: https://github.com/HLLMR/silo/issues

---

_Not affiliated with GIANTS Software. Farming Simulator is a trademark of its
respective owner._
