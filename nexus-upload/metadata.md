# Nexus mod page — field values (copy/paste)

Everything the "Add a mod" form asks for. Description text is separate — paste
`description.bbcode.txt` into the description box **as BBCode** (Nexus is BBCode, not Markdown).

---

## Where it goes (pick one — decide on the "read first" step)
- **Farming Simulator 25 → Tools / Utilities category** — most natural home, since Silo is FS25-specific. Preferred if FS25 exposes a tools/utilities category.
- **Nexus "Modding Tools" (site-wide)** — where cross-game managers live (MO2, Fluffy, Unity MM). Use if FS25 has no tools category or staff direct you there.

## Name
```
Silo — FS25 Mod Manager
```

## Summary (one line, shown in listings — keep ~1 sentence)
```
The management layer FS25 lacks: loadouts, conflict detection, crash triage, catalog + updates, and build-verification — open source and code-signed.
```

## Version
```
0.5.0
```

## Category
Tools / Utilities (see placement note above).

## Tags (pick from Nexus's list where they match; suggestions)
`Utilities`, `Tools`, `Mod Manager`, `Quality of Life`, `Modders Resources` — plus free-text if allowed: conflict-detection, loadouts, updates, multiplayer, provenance.

## Requirements
- Windows 10/11 64-bit (macOS + Linux on GitHub)
- Farming Simulator 25 installed
- No runtime/framework dependency

## Files to upload
- **Main file:** `Silo_0.5.0_x64-setup.exe` (staged in this folder — the signed NSIS installer)
- **Optional:** `Silo_0.5.0_x64_en-US.msi` (staged here — MSI for managed/enterprise installs)
- macOS `.dmg` / Linux `.AppImage` are on the GitHub release if you want to offer them as optional files:
  https://github.com/HLLMR/silo/releases/tag/v0.5.0
- Set the file's own version to `0.5.0` and mark the .exe as the **Main** file.

## Images
- **Main image:** `images/main-image.png`
- **Gallery (in order):** `images/01-library.png` … `07-multiplayer.png`

## Permissions / licensing (it's MIT — be generous, and say so)
- License: **MIT** (state it in the permissions/notes).
- Users can: use freely. Others can build from source (it's public).
- Recommended permission toggles: allow modification, allow use in other mods/tools, allow re-upload of *your own* — but honestly, point everything at the MIT license + GitHub rather than relying on Nexus's matrix.

## Version / changelog (0.5.0)
Paste from `changelog.bbcode.txt`, or:
- ModHub-order "Newest" sort in Browse
- Right-click menu on library mods (incl. Find in Browse)
- Collections split into its own panel
- Shared-collection links are now a real handoff page (not raw JSON)
- Manage your collections in-app: list / copy / open / update / delete
- Signed, checksummed, attested, SBOM'd builds
