# Silo docs

Design, reference, and process material for the (now-built) app. Start with the root
[README](../README.md) and [CONTRIBUTING](../CONTRIBUTING.md); the files below are the
deeper background. Suggested reading order:

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

- **[reference/fs25-modding-notes.md](reference/fs25-modding-notes.md)** — how FS25
  loads mods, `modDesc.xml` fields, conflict signals, projection mechanics.

Plus per-feature notes: `ENRICHMENT.md` (catalog signals), `BASE_SAVES.md`,
`SMOKE_TEST.md` (the release checklist), and `RELEASE_CHECKLIST.md`.
