# Silo docs

Planning and reference material. Read in this order:

1. **[VISION.md](VISION.md)** — the problem, the insight, positioning, competitors.
2. **[MVP.md](MVP.md)** — v0.1 scope (and what's deliberately excluded).
3. **[DESIGN.md](DESIGN.md)** — design language, stack picks, farm color system,
   terminology approach, and the tinkerer power-layer (incl. the XML settings-form
   generator).
4. **[ARCHITECTURE.md](ARCHITECTURE.md)** — Tauri/Rust design, data model, conflict
   engine, the projection engine.
5. **[CROSS-PLATFORM.md](CROSS-PLATFORM.md)** — Windows/macOS/Linux game-file
   discovery and per-OS projection.
6. **[DECISIONS.md](DECISIONS.md)** — why we chose what we chose.

Reference (mined knowledge):

- **[reference/incumbent-teardown.md](reference/incumbent-teardown.md)** — good
  ideas + every do-not-repeat from the existing Electron tool.
- **[reference/fs25-modding-notes.md](reference/fs25-modding-notes.md)** — how FS25
  loads mods, `modDesc.xml` fields, conflict signals, projection mechanics.

No application code yet — this is plan-first. Scaffold order when we start:
`src-tauri` core (scan → moddesc → db) before any UI.
