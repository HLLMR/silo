# Silo — Smoke-Test Checklist (release gate)

Run top-to-bottom in the built app before tagging a release. Each item: **do → expect**.
Tags: `[app]` app-only · `[game]` needs FS25 (and often a loaded save) · `[net]` needs the
SiloAPI catalog reachable. "Auto-verified" = the logic/data path is covered by Rust unit
tests / `svelte-check` in CI; the box here is the *human* GUI/behaviour check that code
can't do.

Launch for testing: from `Silo/`, `npm run tauri:dev` (leave the terminal open — closing it
closes the app). A packaged build is `npm run tauri:build`.

---

## 0. Launch & shell
- [ ] `[app]` App launches, no error dialog, window renders the Library view.
- [ ] `[app]` Library/Browse tabs switch cleanly; theme toggle (Settings) cycles system/light/dark and both themes are legible.
- [ ] `[app]` Resize the window — no horizontal scroll, panels stay centered.

## 1. Library scan & organize  _(auto-verified: scan/organize/moddesc engines)_
- [ ] `[app]` Initial scan lists the full library; counts (mods/maps/scripts/DLC) look right; "scanned in N ms" shows.
- [ ] `[app]` Rescan re-reads without error; warm scan is fast.
- [ ] `[app]` Organize moves loose zips into `mods/archive/<Category>/`; dev **folder** mods are left untouched (zip-only).
- [ ] `[app]` Category rail filters; search filters by title/author/tech name; Favorites/Hidden toggles work.
- [ ] `[app]` Select-all checkbox activates/deactivates the filtered set.

## 2. Curation & detail drawer  _(auto-verified: curation/tags/rating store)_
- [ ] `[app]` Click a mod row → detail drawer opens with icon, metadata, deps (with present/missing status).
- [ ] `[app]` Favorite / Broken / Hidden toggles persist across a rescan; star rating persists; tags add/remove & filter; notes save on blur.
- [ ] `[app]` **Last run** section shows this mod's log health (see §8) — errors/cosmetic/clean.
- [ ] `[net]` **Catalog** section shows latest version / "update available (source)" / "not in catalog".
- [ ] `[app]` Reveal opens the file in Explorer; activate/deactivate toggles the active dot.

## 3. Loadouts & savegames  _(auto-verified: loadout/savegame parse)_
- [ ] `[app]` Save current active set as a loadout; apply/overwrite/delete; active loadout name shows in the toolbar.
- [ ] `[app]` Export a loadout to `.silo`, re-import it as a new loadout.
- [ ] `[game]` Savegames panel lists real saves with map + mod list; "→ Loadout" builds a loadout from a save's mods (warns on missing).
- [ ] `[app]` Savegame backup copies to `SiloBackups/`.

## 4. Conflicts  _(auto-verified: conflict engine incl. map + fillType)_
- [ ] `[app]` Conflicts stat shows count; panel lists uniqueType / specialization / script collisions with severity + involved mods.
- [ ] `[app]` **Duplicate-map**: enable 2+ maps → a critical "N maps enabled" conflict appears; drop to one → it clears.
- [ ] `[app]` **fillType override**: if two active mods declare the same `<fillType name>`, a warning appears naming both; distinct filltypes do NOT flag.

## 5. Health & launch
- [ ] `[app]` "Need attention" panel: missing deps, corrupt mods, digit-prefix names.
- [ ] `[game]` Launch button starts FS25 (Steam) with the active set projected.

## 6. Browse (catalog)  _(auto-verified: adapters, /mods shape)_
- [ ] `[net]` Browse loads the catalog; header shows total; **scroll works** (regression-fixed).
- [ ] `[net]` Search filters; category dropdown filters; "Load more" pages to the end ("That's all N").
- [ ] `[net]` Per-source buttons render with versions: GitHub = ⬇ (install), ModHub/Nexus = ↗ (open page). "In library" badge on owned mods.
- [ ] `[net][game]` Install a GitHub mod → progress bar advances (MB), lands in the library on rescan, then loads in-game.
- [ ] `[net]` Details opens the drawer: cover image, facts, full source list with per-source versions + open-page links.

## 7. Updates
- [ ] `[net]` "⟳ Updates" checks the whole library via the catalog; rows show current→latest with source badge; Install works.
- [ ] `[net]` Detail-drawer GitHub link: paste owner/repo (or accept the auto-suggestion), Check, Install — backs up `.bak`.

## 8. Crash & log triage  _(auto-verified: log parser on real 713-mod log)_
- [ ] `[app]` "◆ diagnose" → verdict banner: healthy / crashed / errors-worth-a-look, matching your last run.
- [ ] `[app]` Culprit mods ranked; cosmetic-only mods behind the "safe to ignore" disclosure; unattributed count + Open log.txt.
- [ ] `[app]` Re-read log after a new FS25 run updates the report.

## 9. Guided bisection  _(auto-verified: stepper convergence 1..64)_
- [ ] `[app]` From triage (crash, no culprit) or the footer link → intro shows suspect count + ~launches.
- [ ] `[game]` Full loop: Start → Silo applies half → Launch FS25 → reproduce → quit → "still broken?" (crash auto-detected) → narrows → repeats → names a culprit (or "interaction").
- [ ] `[app]` Finish/cancel restores your original active set exactly.
- [ ] `[app]` **Crash-safety**: start a bisection, force-quit Silo mid-run, relaunch → recovery banner offers to restore your set; Restore works.

## 10. Bindings map  _(auto-verified: parser on real inputBinding.xml)_
- [ ] `[app]` "⌨ bindings" lists devices; every action↔key shown; search filters; "inputs bound to more than one action" section highlights reuse (as review, not error).

## 11. Multiplayer sync  _(auto-verified: manifest diff + md5 vs md5sum)_
- [ ] `[app]` "Multiplayer" → Export active set → saves a `.silomp`.
- [ ] `[app]` Round-trip: Verify against your own just-exported manifest → "your set matches."
- [ ] `[app]` Negative: deactivate a mod, re-verify the same manifest → it shows under "get these" (and reactivating clears it). Change a version → "wrong version". An extra active mod → "turn these off".

## 12. Filltype bridge  _(auto-verified: generator XML vs real mods)_
- [ ] `[app]` "⛓ bridge" → enter a filltype name, pick categories, Generate → `.zip` appears in mods folder + rescan; it shows in the library.
- [ ] `[game]` **The real test**: enable the bridge + the map, load a save → the filltype now works with the checked equipment (haul/store/sell). If off, delete the zip (reversible).

## 13. Config editors & diagnostics  _(auto-verified: xmlconfig/settings_form)_
- [ ] `[game]` game.xml graphics editor: apply a preset, relaunch → setting took; `.bak` written.
- [ ] `[game]` Savegame config editor: change difficulty/growth etc., load save → applied; `.bak` written.
- [ ] `[app]` Per-mod settings form (for a mod that has settings) edits + saves with `.bak`; raw-XML fallback works.
- [ ] `[app]` Diagnostics report exports a markdown summary.

---

## Sign-off
- [ ] All `[app]` items pass.
- [ ] All `[game]`/`[net]` items pass (or known-limitations noted below).
- [ ] `npm run tauri:build` produces a launchable packaged app.

**Known limitations to note at release:** ModHub/Nexus are index-only (open-page, no direct
install — GIANTS/Nexus gate downloads); ghost-keybind cleanup deferred (unverified premise);
override-conflict detection is fillTypes-only for now.
