# Silo — Design language & experience

The brief: a tool for tinkerers that advanced simmers respect and newcomers aren't
scared of; not a wall of tiny icons and boxes; a genuinely pleasant place to be;
farm/outdoor color theory carried through every decision; and unmistakably the one
that's *doing it right*. This doc is the source of truth for look, feel, stack, and
language.

## Design pillars

1. **Calm, not crowded.** Generous whitespace, few things per view, one clear
   primary action per screen. Depth comes from *progressive disclosure*, not
   density. A newcomer sees a clean library; a tinkerer opens the drawers.
2. **Earn trust visibly.** Every file-touching action shows what it will do and how
   to undo it. Nothing about the game folder happens invisibly. Trust is the moat.
3. **Approachable surface, deep floor.** Plain-language defaults up top; power
   (rules, diffs, graphs, CLI) one layer down. Never make the beginner pay the
   complexity tax, never cap the expert.
4. **Farm-honest aesthetic.** Natural materials and golden-hour light, not neon
   "gamer" chrome and not sterile enterprise gray. It should feel like good tools
   in a clean workshop.
5. **Fast is a feature you can see.** Instant scans, smooth 60fps lists, immediate
   feedback. The incumbent feels broken; we feel *solid*.

## Stack decisions (committed)

- **Shell:** Tauri v2 (Rust core) — decided; see `DECISIONS.md` / `ARCHITECTURE.md`.
- **Frontend framework: Svelte 5 (runes) + Vite + TypeScript.** Why: smallest
  runtime and no virtual-DOM overhead (aligns with "native, fast, tiny"); fine-
  grained reactivity avoids the whole-store re-render storms that sank the
  incumbent; and its low ceremony lets us pour effort into *bespoke* visuals rather
  than fighting a component framework. TypeScript throughout for a tinkerer-friendly,
  refactorable codebase.
- **Styling: hand-authored CSS with design tokens** (CSS custom properties), no
  heavyweight UI kit. The farm aesthetic must be ours, not a themed Bootstrap. A
  tiny set of primitives (Button, Card, Field, Toggle, Tag, Toast) built in-house.
- **Icons:** Lucide, used **sparingly and always with a label** where it matters.
  Icons accent meaning; they don't replace words.
- **Charts/graphs (later):** lightweight SVG / the `dataviz` approach — for the
  dependency graph, disk-usage, and library-health views.
- **Fonts (self-hosted, offline-bundled):** display/headings **Fraunces** (a warm,
  slightly rustic humanist serif — craft and character); UI/body **Hanken Grotesk**
  or **Inter** (clean, legible, neutral). Numbers **tabular** in all data views.
- **Motion:** gentle, natural easing (wind, not springs). 150–250ms. Respect
  `prefers-reduced-motion`.

## Color system — "Golden Hour on the Farm"

Farm/outdoor color theory: earth, growth, and natural light. Warm neutrals (soil,
linen, oat) instead of cold grays; **field green** as the living primary; **harvest
gold** as the accent that draws the eye; **sky blue** for calm information;
**barn red / terracotta** for danger — warm, never a fire-engine alarm. Two themes
are one idea at two times of day.

### Neutrals (warm — soil & linen, never pure gray)
| Token | Light (Daybreak) | Dark (Dusk) | Use |
|---|---|---|---|
| `--bg` | `#F6F3EC` oat | `#171A14` loam | app background |
| `--surface` | `#FCFAF4` linen | `#1F231C` tilled earth | cards, panels |
| `--surface-raised` | `#FFFFFF` | `#282D24` | menus, modals |
| `--border` | `#E4DECE` | `#39402F` | hairlines, dividers |
| `--text` | `#2A2A22` | `#EDEBE0` | primary text |
| `--text-muted` | `#6B6A5C` | `#A8A897` | secondary text |

### Brand & semantic scales (anchor values; build 50→900 ramps in tokens.css)
| Role | Color | Hex | Meaning |
|---|---|---|---|
| **Primary — Field green** | pasture | `#5C8A3A` | growth, "go", the brand |
| | field (600) | `#4A7330` | primary buttons, active nav |
| | forest (700) | `#395A25` | pressed, emphasis |
| **Accent — Harvest gold** | wheat | `#E3B23C` | highlight, focus, "new", CTA glow |
| | amber (600) | `#C9922A` | hover on accent |
| **Info — Sky** | sky | `#4A90C2` | neutral info, links, "syncing" |
| **Warn — Squash** | squash | `#D98A1F` | caution, out-of-date, soft conflict |
| **Danger — Barn red** | barn | `#B34A38` | destructive, critical conflict |
| **Earth — Soil brown** | soil | `#8A6A45` | tertiary warmth, disabled-warm |

### Rules
- **Never encode meaning by color alone.** Conflict severity, status, and tags
  always pair color with an icon/shape/label (FS players include colorblind users;
  the game ships a colorblind mode — we honor that). Provide a colorblind-safe
  palette variant.
- Target **WCAG AA** (4.5:1 body, 3:1 large/UI). Validate every text-on-surface
  pair in both themes; the anchors above are chosen to pass but ramps must be
  checked.
- Harvest gold is the **scarce** color — reserve it for the single thing that most
  wants attention on a screen. Overusing it kills its power.
- Dark theme is warm dark (loam), **not** black. Elevation = getting lighter/warmer.

## Layout & density

- **8px spatial grid**; comfortable, roomy. Real padding on cards; no cramped rows.
- Persistent left **nav rail** (few, clear destinations), a calm top bar with the
  one primary action, content in the center with breathing room.
- Mod cards: image-forward, title legible, *at most* 2–3 metadata chips visible;
  everything else on the detail view. No card should look like a spreadsheet cell.
- Empty states are a feature: warm, illustrated-ish, they teach the next step in
  plain, translatable language.

## Language & terminology

Follow the GIANTS in-game approach: **plain, literal words that translate cleanly
into dozens of languages.** No invented vocabulary, no metaphor decoder ring. Use
the obvious terms — *library, loadout, apply, clear, conflict, health check,
updates, savegame mods, favorite, hidden, broken.* "Silo" is the app's working
name only; it is not a term users have to learn.

Terminology is deliberately **not finalized here** — final wording (and any light
flavor) will be refined by hand once the app is built and every string is a
translation key. Until then, code and UI use plain descriptive names.

## Give the tinkerers things to do (power layer)

The floor under the calm surface — this is what makes advanced simmers adopt it and
evangelize it:

- **XML settings, rendered as a real form.** Most mods expose tunable options only
  as hand-edited XML. In each mod's tinkerer area, Silo reads the mod's settings
  XML and **generates a clean, stylized, editable form** — toggles, sliders, number
  fields, dropdowns — inferred from the values (and the XSD where one exists), then
  writes changes back to the XML safely. See `ARCHITECTURE.md#settings-form-generator`.
  This is the flagship tinkerer feature: it turns "open the file in Notepad" into a
  first-class UI.
- **Loadouts you can compose & compare.** Build, clone, and **diff two loadouts**
  (what mods/versions differ). Import/export a loadout as a shareable file.
- **Conflict explorer.** Every conflict drills down to the exact `modDesc.xml`
  element and the two mods involved (shared specialization/type/uniqueType/script),
  with a plain-English "why this clashes."
- **Health-check rules engine.** User-defined rules over the library: e.g. *tag any
  mod >200 MB "heavy," flag mods sharing a `<uniqueType>`, warn on `extraSourceFiles`
  overlap, surface digit-prefixed names.* Ship a strong default rule-pack; let
  power users add their own.
- **Dependency graph.** Visualize the active loadout's dependency web; highlight
  missing/optional deps and the `required="true"` mods a chosen save needs.
- **Snapshots & rollback.** Snapshot the mods-folder state before any change; one-
  click restore. (Reversibility made tangible.)
- **Savegame binding.** Generate a loadout from a save's mod list; verify a save's
  `required` mods are present and version-matched (via `fileHash`).
- **Saved filters / queries** over the library (by author, category, size, age,
  MP-support, "has scripts," "touches shop"). Bulk actions on the result set.
- **CLI / headless mode.** Apply a loadout, run a health check, export a report from
  a script — for the automation-minded. (The Rust core makes this cheap: the GUI and
  CLI call the same engine.)
- **Per-mod notes, personal rating, and a changelog trail.** The curation memory
  the community lacks today.

## What "doing it right" looks like at a glance

Set beside the incumbent, the first ten seconds should read as: it opened
instantly; the library is calm and beautiful, not a grid of tiny boxes; it clearly
knows my mods (icons, versions, conflicts) without me configuring anything; and the
one big warm-gold button tells me exactly what to do next. Fast, honest, handsome.
