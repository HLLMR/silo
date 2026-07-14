# Decisions log

Lightweight ADRs. Newest first. One entry per decision that would be expensive to
reverse or that a newcomer would ask "why?" about.

## 2026-07-14 — Build our own, don't PR or fork the incumbent
**Context:** `MarkThor11/fs25-mt-mod-manager` exists and has the right core idea.
**Decision:** Build fresh. **Why:** its problems are architectural (main-thread
blocking, whole-store re-renders, scraping backbone, self-obfuscation), not
spot-fixable; and there's no clean fork point ("initial commit" is already the full
monolith, 27 commits over 5 days). We harvest its ideas as spec instead
(`reference/incumbent-teardown.md`).

## 2026-07-14 — Stack: Tauri (Rust core)
**Context:** the incumbent's headline weakness is speed/stability from doing heavy
work on a JS UI thread. **Decision:** Tauri v2, Rust core, web frontend. **Why:**
moves archive parsing/hashing/image-decode/tree-walks to native threads by
construction; ~10 MB binary; "native, fast, tiny" is a direct competitive wedge.
**Cost accepted:** Rust learning curve, rewriting FS parsing in Rust.

## 2026-07-14 — No ModHub scraper in v1
**Context:** the incumbents pour effort into scraping GIANTS' site (fragile,
ToS-gray, bot-evasion) while the management layer stays half-baked. **Decision:**
v1 management works fully offline; scraping/browsing is a possible later add-on.
**Why:** management is the sticky, defensible moat; discovery is replaceable and
GIANTS could kill it.

## 2026-07-14 — Frontend: Svelte 5 + Vite + TypeScript
**Context:** need bespoke, beautiful UI and zero re-render-storm risk. **Decision:**
Svelte 5 (runes) + Vite + TS, hand-authored CSS design tokens, no heavyweight UI
kit. **Why:** smallest runtime (fits "native, fast, tiny"); fine-grained reactivity
avoids the incumbent's whole-store re-render failure by construction; low ceremony
frees effort for craft. See `DESIGN.md`.

## 2026-07-14 — Cross-platform: Windows + macOS + Linux
**Context:** "these are tinkerers like us" across OSes; FS25 ships Win + Mac, Linux
via Proton. **Decision:** support all three natively (Tauri targets), with
per-OS game-file discovery and projection strategy. **Why:** the tinkerer audience
is cross-platform; Tauri makes it cheap; detection (Steam `libraryfolders.vdf`,
Epic, GIANTS) + manual override covers real installs. See `CROSS-PLATFORM.md`.

## 2026-07-14 — Ground the data model in the real SDK schema
**Context:** the incumbent guessed modDesc fields via regex and shipped bugs.
**Decision:** model straight from `modDesc.xsd` / `gameSettings.xsd` /
`careerSavegame.xml` (mirrored locally, gitignored to respect GIANTS' IP). Key
facts: `<dependency>` is a tech-name **string**; `<uniqueType>` is an explicit
conflict primitive; six namespace surfaces drive conflict detection; saves carry
`required` + `fileHash` per mod. See `reference/fs25-modding-notes.md`.

## 2026-07-14 — Working title "Silo"
**Context:** need a name comparable-in-ambition to Nexus Mod Manager but without
"mod manager" in it. **Decision:** working title **Silo** (a silo = organized,
sealed, retrievable farm storage). Revisit before any public release.
