# Silo — showcase (Tier B) + hook (Tier A) scripts

The two you launch with. See [`VIDEO_PLAN.md`](./VIDEO_PLAN.md) for how they fit the ladder and the
footage vault (V1–V13) these cut from.

- **Tier B — the showcase:** ~2.5–3.5 min, screen capture + **word-for-word** VO. Short VO wants
  precision (unlike the rambly [walkthrough](./demo-video-script.md)). This is the landing-page hero
  and the "what is this" link.
- **Tier A — the hook:** 20–45s vertical cut of the crash-diagnosis moment, for Shorts/Reels/TikTok.

VO is written to be **said, not read** — trim to your natural cadence. `[SHOW]` = what's on screen.
Keep the cursor deliberate; click, beat, then talk.

---

## Tier B — the showcase (~3 min)

### 0:00–0:12 · Cold open (the pain)
`[SHOW]` FS25 crashing on load, or a wall-of-red `log.txt`, or your enormous `mods/` folder. Hard cut
to the Silo Library on your real count.
> "If you run Farming Sim with a stack of mods, you know this: the game dies on load and won't tell
> you which one did it. This is Silo — that's my actual library, seven hundred–plus mods."

### 0:12–0:55 · The money shot — crash diagnosis  *(V1)*
`[SHOW]` Click **◆ diagnose**. Let it parse and name the culprit. Open that mod, hit the **park**
switch. Show the "healthy" state. If you can, relaunch into the game.
> "When it crashed, I didn't disable mods half at a time and pray. I hit diagnose. Silo reads the
> game's log, throws out the cosmetic noise, and points at the mod most likely at fault. One click to
> park it — relaunch — and we're in the game."

`[SHOW — optional, 5s]` `⚙ bisection` disabling half.
> "If the log's not clear, it automates the disable-half-and-relaunch hunt for you — and it snapshots
> your setup first, so you can't lose it."

### 0:55–1:25 · Catch it *before* launch — conflicts  *(V5)*
`[SHOW]` Open **conflicts**; land on a real one (duplicate active map, or a filltype/script clash).
> "Better than fixing a crash is not having one. Two active maps is an instant crash on load — Silo
> catches that before you ever hit Launch, along with vehicle-type, filltype, and script clashes that
> quietly collide across your set."

### 1:25–2:00 · Is this the real build? — provenance  *(V6)*  ← the moat
`[SHOW]` A mod's detail drawer → **integrity check** → **✓ Verified** (and a **Modified** with the
changed files if you have one).
> "Here's the part I haven't seen anywhere else. Silo hashes your installed mod and checks it against
> the real build its source published. Verified means byte-for-byte identical. Modified tells you
> exactly which files differ. It's provenance, not antivirus — it proves *what* a mod is, and it
> works across ModHub and GitHub."

### 2:00–2:30 · One catalog, every source + updates  *(V7, V8)*
`[SHOW]` **Browse**; search a mod; the merged ModHub/GitHub record with the latest version. Then
quick: drop a new zip in → the mod drawer's **Adopt** banner.
> "A mod might live on ModHub, get newer builds on GitHub. Silo folds them into
> one record so you see the actual latest — not whichever site updated last. And when you drop a new
> version in to update, Silo spots it and offers to adopt it in a click."

### 2:30–2:55 · "…and it also" — rapid montage  *(V9, V10, V12)*
`[SHOW]` Fast cuts, ~4s each: Loadouts swap · Collections share-link · multiplayer verify · filltype
bridge.
> "Save a mod set per playthrough and swap it in one click. Share your exact setup as a link. Verify
> everyone's mods match before a co-op session. There's a lot in here."

### 2:55–3:20 · Trust + CTA
`[SHOW]` Nothing fancy — the app, then `silo.hllmr.com`.
> "It's free and open source — every line's on GitHub. No account, no telemetry, and everything it
> touches on your files is reversible; it never deletes something it can't prove it made. It's a
> beta, Windows-first. Grab it at silo dot hllmr dot com, and tell me which mod folder finally made it
> fall over."

**Notes:** Total ~3:20; cut the optional bisection line to land under 3:00. Keep the provenance beat —
it's the differentiator. Auto-caption and proofread mod names. Publish **unlisted** first, get Discord
eyes, then public.

---

## Tier A — the hook (20–45s vertical)  *(V1)*

Same crash-diagnosis moment, framed 9:16, on-screen text carrying the story for silent autoplay. Cut
three so you can A/B which travels.

### A-1 · "Which mod crashed you?" (≈20s)
`[SHOW]` FS25 crash → hard cut to Silo → **◆ diagnose** → culprit named → park → healthy.
- On-screen text beats: **"FS25 crashed on load."** → **"700 mods. Which one?"** → **"Silo read the
  log."** → **"→ [Mod Name]"** → **"Parked it. Back in the game."**
- VO (optional): "It crashed, the game shrugged, so I let Silo read the log — and it named the one."

### A-2 · The number flex (≈30s)
`[SHOW]` Slow pan of the huge library, count visible → diagnose → name → relaunch into game.
- Text: **"I run 700+ FS25 mods."** → **"One crashes. Good luck finding it."** → **"Silo names it in
  seconds."** → **"Free. Open source. silo.hllmr.com"**

### A-3 · Provenance angle (≈30s, for the OSS/mod-author crowd)
`[SHOW]` Integrity check → Verified → then a Modified with changed files.
- Text: **"Is that mod the real build?"** → **"Silo hashes it vs the source."** → **"✓ Verified =
  byte-for-byte."** → **"Modified = here's what changed."** → **"Provenance, not antivirus."**

**Hook rules:** first frame is the payoff-in-progress (no slow intro); big legible captions; end on the
URL for ≥2s; ≤3 words of jargon. See [`tiktok-instagram.md`](./tiktok-instagram.md) for aspect/caption
repurposing and the `crash-clip-factory` skill.
