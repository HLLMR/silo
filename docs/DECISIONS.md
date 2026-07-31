# Decisions log

Lightweight ADRs. Newest first. One entry per decision that would be expensive to
reverse or that a newcomer would ask "why?" about.

## 2026-07-14 — Ground-up Rust/Tauri architecture
Chose a native stack over an Electron approach for the performance target.

## 2026-07-14 — Stack: Tauri (Rust core)
**Context:** speed/stability suffer when heavy work runs on a JS UI thread.
**Decision:** Tauri v2, Rust core, web frontend. **Why:**
moves archive parsing/hashing/image-decode/tree-walks to native threads by
construction; ~10 MB binary; "native, fast, tiny."
**Cost accepted:** Rust learning curve, rewriting FS parsing in Rust.

## 2026-07-14 — No ModHub scraper in v1
**Context:** scraping GIANTS' site is fragile, ToS-gray, and needs bot-evasion,
while the offline management layer is the higher-value work. **Decision:**
v1 management works fully offline; scraping/browsing is a possible later add-on.

## 2026-07-14 — Drop the thematic lexicon; use plain, translatable language
**Context:** an earlier pass invented a farm-metaphor vocabulary (Fields/Weeds/Soil
Test/…). **Decision:** remove it. Follow GIANTS' in-game approach — plain literal
words that translate cleanly into dozens of languages (library, loadout, apply,
clear, conflict, health check, updates). Terminology is **not finalized**; final
wording refined by hand once built and every string is a translation key. "Silo"
remains only the app's working name. **Why:** a decoder-ring vocabulary hurts
approachability and i18n; the user will own the language pass.

## 2026-07-14 — XML settings form generator (tinkerer feature)
**Context:** most mods expose tunable options only as hand-edited XML. **Decision:**
each mod's tinkerer area generates a stylized, editable form from its settings XML
(XSD-assisted where available), writing back with a minimal diff + original backup,
with a raw-XML escape hatch. New `settings/` Rust module + generic Svelte renderer;
MVP item 7. **Why:** turns "edit in Notepad" into first-class UI — a headline
differentiator. Design in `ARCHITECTURE.md#settings-form-generator`.

## 2026-07-14 — Frontend: Svelte 5 + Vite + TypeScript
**Context:** need bespoke, beautiful UI and zero re-render-storm risk. **Decision:**
Svelte 5 (runes) + Vite + TS, hand-authored CSS design tokens, no heavyweight UI
kit. **Why:** smallest runtime (fits "native, fast, tiny"); fine-grained reactivity
avoids whole-store re-render failures by construction; low ceremony
frees effort for craft. See `DESIGN.md`.

## 2026-07-14 — Cross-platform: Windows + macOS + Linux
**Context:** "these are tinkerers like us" across OSes; FS25 ships Win + Mac, Linux
via Proton. **Decision:** support all three natively (Tauri targets), with
per-OS game-file discovery and projection strategy. **Why:** the tinkerer audience
is cross-platform; Tauri makes it cheap; detection (Steam `libraryfolders.vdf`,
Epic, GIANTS) + manual override covers real installs. See `CROSS-PLATFORM.md`.

## 2026-07-14 — Ground the data model in the real SDK schema
**Context:** guessing modDesc fields via regex ships bugs.
**Decision:** model straight from the real XSD — `modDesc.xsd` / `gameSettings.xsd` /
`careerSavegame.xml` (mirrored locally, gitignored to respect GIANTS' IP). Key
facts: `<dependency>` is a tech-name **string**; `<uniqueType>` is an explicit
conflict primitive; six namespace surfaces drive conflict detection; saves carry
`required` + `fileHash` per mod. See `reference/fs25-modding-notes.md`.

## 2026-07-14 — Working title "Silo"
**Context:** need a name comparable-in-ambition to Nexus Mod Manager but without
"mod manager" in it. **Decision:** working title **Silo** (a silo = organized,
sealed, retrievable farm storage). Revisit before any public release.
