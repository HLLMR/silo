# FS25 modding notes (for Silo's engine)

Ground-truth facts about how FS25 loads and structures mods, that Silo's scan /
conflict / projection engines depend on. Sourced from the game's behavior, the
SDK, and cross-checked against the incumbent's parser
(`../fs25-mt-mod-manager/src/main/services/modManager.js`). **Verify anything
marked (?) against the SDK `modDesc.xsd` before relying on it in code.**

## File locations (Windows)

- **Game user dir:** `Documents/My Games/FarmingSimulator2025/`
  - `mods/` — the flat folder FS25 actually reads. A mod is either `Foo.zip` or an
    unpacked `Foo/` dir containing `modDesc.xml`.
  - `gameSettings.xml` — contains `<modsDirectoryOverride active="..." directory="..."/>`
    for a custom mods path. Silo should read this to discover the real mods root.
  - `savegame1/` … `savegameN/` — each has `careerSavegame.xml` (+ `farms.xml`,
    `vehicles.xml`, etc.). The save's active mod list lives here.
  - `pdlc/` — paid DLC (separate from mods).
- **Game install:** e.g. `E:\SteamLibrary\steamapps\common\Farming Simulator 25`.
  May also hold a `pdlc/` for Steam/Epic DLC.

## How FS25 decides what to load

- It scans **only the flat `mods/` root** — no subfolder recursion. This is *why*
  organizing means "one giant folder" today, and why Silo projects a chosen set in.
- **Mods whose name starts with a digit are ignored by the engine.** A folder/zip
  named `07_FooMap.zip` silently won't load. Silo must flag this as a health issue
  (the incumbent skips them at scan with a warning — `modManager.js:940`).
- Whether a mod is "active" for a given save is recorded in that save's
  `careerSavegame.xml` mod list (modName + optional version/title). Loading a save
  expects those mods present in the folder.

## `modDesc.xml` — the fields Silo parses

Root: `<modDesc descVersion="…">`. Relevant children (parse with `quick-xml`):

- `<author>` — text.
- `<version>` — dotted version string (e.g. `1.0.0.0`). Use for real update compare.
- `<title>` — localized; may be `<title><en>…</en><de>…</de></title>` or a `title=`
  attribute. Prefer `en`, fall back to any language, then the folder name.
- `<description>` — localized similarly.
- `<iconFilename>` (a.k.a. iconFile) — path to the mod icon inside the archive,
  usually a `.dds` (sometimes `.png`). 256/512 square. Silo decodes off-thread.
- **Maps:** `<maps><map id="…" title="…" className="…" filename="…"/></maps>`, or a
  `<map>` block. Presence of a map + a map `.i3d` reference ⇒ treat as a map mod.
  Maps are large (100 MB+ zips) — don't load the whole archive for the icon.
- **Dependencies:** `<dependencies><dependency modName="FS25_Foo" url="…">…</dependency>`.
  `modName` is the required mod's tech name; `url` (when present) may embed a
  ModHub id (`mod_id=NNN` or `storage/NNN/`). Model as a struct
  `{ mod_name, url?, mod_id? }` — **never assume it's a bare string** (that
  assumption is the incumbent's blank-library crash).
- **Scripts:** `<extraSourceFiles><sourceFile filename="scripts/Foo.lua"/></extraSourceFiles>`.
  Two active mods contributing the same script basename ⇒ conflict signal.
- **Specializations:** `<specializations><specialization name="fooSpec" className="…" filename="…"/></specializations>`.
  Duplicate `name` across active mods ⇒ **critical** conflict (register clash).
- **Store items:** `<storeItems><storeItem xmlFilename="…"/></storeItems>` — points
  at vehicle/placeable XML; source of item counts and (for vehicles) tech specs.
- `<l10n>` — localization; filename/id collisions are a softer conflict signal.

The mod's **tech name** is its folder/zip basename (e.g. `FS25_RealisticFoo`) and is
the identity the game and dependencies use — more stable than the display title.

## Conflict signals (for the detection engine)

Ranked rough severity — validate against the 700-mod corpus before shipping:

| Signal | Severity | Source |
|--------|----------|--------|
| Duplicate `<specialization>` name across active mods | critical | modDesc |
| Same tech name present twice (dup install / version clash) | critical | folder |
| Explicit `<dependency>` missing from active set | high | modDesc |
| Duplicate script basename in `<extraSourceFiles>` | warning | modDesc |
| Same `<storeItem>`/store category id collision | warning (?) | store XML |
| l10n / input-binding id collision | info/warning (?) | modDesc / xml |
| Mod name starts with a digit (engine ignores it) | health-warning | folder |
| Corrupt/unreadable zip, missing modDesc.xml | health-error | scan |

## Projection mechanics (Windows)

- **Directory junctions** (`fs::junction` / `mklink /J`) generally work **without**
  admin or Developer Mode — good for unpacked mod dirs.
- **File symlinks** (for `.zip` mods) typically need **Developer Mode or admin**.
  This is the exact seam where the incumbent is "buggy" for some users.
- **Hardlinks** work for files but **fail across volumes** (library on D:, game on C:).
- ⇒ Silo detects capability per game-root once, and falls back to **copy-mode** as a
  universal, first-class path. Track every link/copy Silo creates so cleanup only
  ever removes Silo-owned entries — never user files.

## Reference corpus & fixtures

- `Documents/My Games/FarmingSimulator2025/mods/` — 700+ real mods. Primary test
  fixture for scan performance and conflict-detection accuracy. Good economy/complex
  analogs to include in tests: large map mods, script mods with specializations,
  mods with URL-form dependencies (the crash case), digit-prefixed names.

## To verify against the SDK before coding

- [ ] Exact `modDesc.xsd` element/attribute names (esp. `iconFilename` vs
      `iconFile`, `<maps>` schema, store-item attributes).
- [ ] `careerSavegame.xml` mod-list element shape (modName/version/title fields).
- [ ] Whether FS25 dedupes by tech name or by folder name when both `.zip` and dir
      exist.
- [ ] Current `gameSettings.xml` `modsDirectoryOverride` exact attributes.
