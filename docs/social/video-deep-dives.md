# Silo — deep-dive series (Tier D)

Talking-head + in-game YouTube episodes, split by **job-to-be-done** for retention. See
[`VIDEO_PLAN.md`](./VIDEO_PLAN.md) for where this sits and the footage vault (V1–V13). These are
looser than the [showcase](./video-showcase-script.md) — you're on camera, using Silo inside a real
modded playthrough, talking like you're helping a friend. **Record Ep 1 first.**

Format per episode: **talking-head cold open** (your face, ~15–30s) → screen-capture + voiceover for
the work → **talking-head trust beat + CTA** to close. 10–18 min each. One clear job, done end to end
on your real library.

---

## Shared blocks (reuse in every episode)

**Cold open (talking head, 15–30s).** Name the pain in the viewer's words before you touch the app.
> "If you play modded Farming Sim, you've hit this: [the episode's specific pain]. I got tired of it
> and built a tool called Silo. Let me show you exactly how I deal with this now."

**Trust beat (talking head or over B-roll, ~30s — near the end).**
> "Quick honesty, because it matters near your game files: Silo is free and open source — the whole
> app is on GitHub. No account, no telemetry. Everything it changes on disk is reversible, and it
> never deletes a file it can't prove it created. And yeah — I built this with a lot of AI help. The
> architecture, the security calls, the testing, the releases, those are mine and human-reviewed. The
> source is right there; don't take my word for it, read it."

**CTA (talking head, ~15s).**
> "It's a beta, Windows-first — Mac and Linux are experimental. Grab it at silo.hllmr.com, come break
> it in the Discord, and drop a comment with the mod that finally crashed *you*. If this helped, it
> helps the channel if you sub — I'm making one of these per job Silo does."

**Recurring visual:** open on your real ~730-mod count once per episode; it earns trust instantly.

---

## Ep 1 — "Fix an FS25 crash in 2 minutes"  *(V1, V2, V5)*  ← record first
**Pain:** the game dies on load and won't say which mod. Most-searched FS25 problem; your hero.

1. **Cold open** — "Game crashes on load, no error, 700 mods. Here's how I find the culprit now."
2. **The manual hell (10s)** — name what everyone does: disable half, relaunch, repeat. Sets up the
   contrast.
3. **Diagnose (V1)** — trigger/reuse a real crash. `◆ diagnose` reads `log.txt`, drops cosmetic
   noise, names the likely culprit. Open it, **park** it, relaunch *into the game on camera*. Let it
   land.
4. **When the log won't say (V2)** — `⚙ bisection`: Silo disables half, you relaunch, it narrows.
   Stress the **crash-safe snapshot** (your set is restored no matter what).
5. **Prevent the next one (V5)** — `conflicts`: show a real duplicate-active-map (instant crash) or a
   script/filltype clash caught *before* launch. Explain severity briefly.
6. **Trust beat + CTA.**

**Thumbnail/title:** "Which mod crashed your Farming Sim? (find it in 2 min)". Chapters on every step.

---

## Ep 2 — "Manage 700 mods without the chaos"  *(V3, V4, V8, V9, V10, V13)*
**Pain:** a flat `mods/` folder you can't reason about; updating and swapping sets by hand.

1. **Cold open** — "Once you pass ~100 mods, the folder becomes unmanageable. This is how I keep 700
   sane."
2. **Library model (V3)** — tiles, the **green active switch**, active vs parked. The big idea: your
   whole library lives filed away; the game only sees the set you mark active, projected in with
   links — no duplication, fully reversible.
3. **Organize (V4)** — the read-only **dry-run** before anything moves ("12 into Vehicles…"). Mention
   the loose-mod case: parking one files it into the library.
4. **Updates + adopt (V8)** — `⟳ Updates` / the **Needs-update** filter finds what's outdated; then
   the real ModHub flow: drop the new zip in, open the mod, **Adopt** the newer build. Done.
5. **Loadouts (V9)** — a set per playthrough; swap the whole thing in one click.
6. **Collections (V10)** — publish your set as a **link** (secret gist); open a friend's link and show
   the import **preview + heads-up** (what you'd need to install). The "here's my exact setup" story.
7. **B-roll callouts (V13)** — non-modal drawer (click a card behind to switch), Find in Library ↔
   Browse. Keep light.
8. **Trust beat + CTA.**

**Title:** "How I manage 700+ Farming Sim mods (without losing my mind)".

---

## Ep 3 — "Is that mod actually safe?"  *(V6)*  ← the differentiator
**Pain:** you download a mod — is it the real build, or has something been slipped in? No one else
answers this.

1. **Cold open** — "You grab a mod from a random mirror. Is it the real thing? Here's how I check."
2. **Provenance (V6)** — a mod's drawer → **integrity check**. Walk all three verdicts: **✓ Verified**
   (byte-for-byte the source's build), **Modified** (show the *exact* changed files), **Unverified**
   (no trusted build to compare yet — **not** a warning).
3. **The honest framing** — "Provenance, not antivirus. It proves *what* a mod is; it doesn't guess
   intent. Unverified isn't bad — it just means the source isn't in the catalog yet. Folder mods can't
   be hashed at all." Say this plainly; over-claiming here torches author goodwill.
4. **Why it's hard/unique** — a cross-source canonical-hash catalog is the moat; nobody else has it.
5. **Trust beat + CTA** — this episode *is* the trust story; lean in.

**Title:** "Is that Farming Sim mod the real build? (how to actually verify it)".

> Caution on camera: never call a legit mod "unsafe." If a viewer disputes a Modified result, that's a
> catalog-data question — point them at the repo/issues, don't argue intent.

---

## Ep 4 — "Co-op without the version roulette"  *(V11, V10)*
**Pain:** you and friends can't join because someone's mods don't match, and no one knows whose.

1. **Cold open** — "Co-op Farming Sim, and the join fails because mods don't match. Here's the fix."
2. **Multiplayer sync (V11)** — host exports a **hashed manifest** of the active set. A joiner verifies
   theirs and gets a **fix-list**: missing / wrong version / different file / extra. Show a real
   mismatch resolving.
3. **Collections for the whole group (V10)** — publish the host's set as a link so everyone imports the
   same list up front; the preview tells each person what they still need.
4. **What it can/can't check** — be honest about the boundaries (it checks the set, not in-game
   behavior).
5. **Trust beat + CTA.**

**Title:** "Fix Farming Sim co-op mod mismatches for good".

---

## Ep 5 *(optional)* — "The weird fixes"  *(V12)*
**Pain:** the long-tail annoyances — a map's filltype your equipment won't accept, tangled keybinds, a
save setting you can't reach without launching.

1. **Cold open** — "A few small Silo tools that fix oddly specific Farming Sim headaches."
2. **Filltype bridge (V12)** — the sugar-beet fix: generate a tiny companion mod so your equipment
   accepts a map's stubborn filltype — no vehicle edits.
3. **Bindings map** — your whole control map in one place, reused inputs flagged. A view, not a verdict.
4. **Savegame editing** — tweak a save's settings (and graphics/`game.xml`) without launching.
5. **Trust beat + CTA.**

**Title:** "5 Farming Sim problems Silo quietly fixes".

---

## Series notes

- **Chapters/timestamps** on every episode (retention + SEO). Titles mirror how people search
  ("Farming Sim mod crash," "co-op mods don't match").
- **Cross-link** episodes in cards/end screens; each closes into the next job.
- **Repurpose** every episode into 1–2 Shorts (a single beat) via the `crash-clip-factory` /
  `cross-post-repurpose` skills — feeds Tier A.
- **Cadence:** one episode per push, not a burst. Better to ship Ep 1 well than four rough cuts.
- **Consistency:** same lower-third, same Golden-Hour palette, same cold-open/trust/CTA rhythm so it
  reads as a series.
