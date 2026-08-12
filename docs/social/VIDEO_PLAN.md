# Silo — video plan (the reference)

The master plan for Silo's demo/marketing video. Read this first; it tells you **which videos
exist, what each is for, and where they live**, then points at the per-video scripts. Grounded in
the positioning + voice guide in [`README.md`](./README.md) and the channel plan in
[`youtube.md`](./youtube.md). Current app: **v0.7.x beta** (feature set below reflects it).

**North star:** lead with the crash-diagnosis ("which mod crashed you?"), prove the fix in one
glance, be honest about beta/AI. Real screen + your real voice — no synthetic avatar or AI VO on
hero content (that's what legitimizes an AI-assisted product).

---

## The ladder — four videos, one footage vault

Don't make "a video." Make four tiers that serve different viewers and **reuse the same capture**.

| Tier | Video | Length | Format | Primary home | Script |
|---|---|---|---|---|---|
| **A** | **The hook** — crash diagnosis only | 20–45s **vertical** | screen cap, punchy VO | Shorts / Reels / TikTok | [showcase script](./video-showcase-script.md#tier-a--the-hook-2045s-vertical) |
| **B** | **The showcase** (your "primary") — top-5 overview | 2.5–3.5 min | screen cap + **word-for-word** VO | landing hero, pinned social, "what is this" link | [showcase script](./video-showcase-script.md) |
| **C** | **The walkthrough** — honest full tour | 10–12 min | narrated screen-share, rambly | evergreen YT "how it works" | [demo-video-script.md](./demo-video-script.md) |
| **D** | **Deep-dives** — in-game, real use | 10–18 min each | **talking head** + capture | your YT channel (a series) | [video-deep-dives.md](./video-deep-dives.md) |

**Why the ladder:** **A travels** (reach), **B converts** (the one link you send), **C educates**
the already-curious, **D builds the channel and trust** (your face on an AI-assisted product is the
credibility play). They nest: A is a cut of B's opening; B's beats are compressed from C; D expands
one C beat into a real playthrough.

**Production order:** shoot the raw capture **once, well** (see Footage vault) → cut A + B first
(they're what you launch with) → post C → record D episodes one at a time as you play.

---

## The footage vault (capture once, cut many)

Record these clean takes on your **real ~730-mod library** at 1080p, 30–60fps, Silo maximized,
Settings/account panels closed (no personal handle on screen), notifications off. Every tier is cut
from this set — you should almost never re-shoot.

| # | Clip | Notes for the take |
|---|---|---|
| V1 | **Crash → diagnose → park → relaunch** | The money shot. Real `log.txt` from a session that actually crashed. Let `◆ diagnose` name the culprit; park it; show the count/"healthy" change; relaunch into the game if you can. |
| V2 | **Guided bisection** | For when the log can't name it: disable-half → relaunch → narrow. Mention the crash-safe snapshot. |
| V3 | **Library at a glance** | The tile grid with your mod count; toggle a mod's **green active switch**; the archive-vs-flat idea. |
| V4 | **Organize dry-run** | The read-only preview ("12 into Vehicles, 3 into Maps") before anything moves. |
| V5 | **Conflicts** | A real one from your set — duplicate active map (instant-crash) or a filltype/script collision. |
| V6 | **Provenance / integrity** | A mod's drawer → integrity check → **✓ Verified** (and a **Modified** with changed files if you have one). |
| V7 | **Browse one catalog** | Search a mod; show ModHub/GitHub folded into one record with the latest version; a GitHub in-app install with the progress bar. |
| V8 | **Update by drop → Adopt** | Drop a newer ModHub `.zip` into the folder; open the mod; the drawer's "newer build — Adopt vX" banner; adopt; version updates. |
| V9 | **Loadouts** | Save the current set; switch to another; watch the active set change. |
| V10 | **Collections** | Publish the active set as a link (secret gist); open a shared link and show the preview + "heads-up" (what you'd need to install). |
| V11 | **Multiplayer sync** | Export the hashed manifest; the joiner's verify/fix-list view. |
| V12 | **Rapid-fire tools** | Savegame settings edit; filltype bridge generate; bindings map. |
| V13 | **B-roll** | Non-modal drawer (click a card behind it to switch), Find in Library ↔ Browse, Needs-update filter, the "healthy" library. Filler between beats. |

> Capture continuously as you actually use Silo — a *genuine* "it named my crash" moment beats any
> staged take. Keep raw takes in a clip vault (see `README.md` pre-launch checklist).

---

## What changed since the last shot list (gap analysis)

The old [`demo-video-script.md`](./demo-video-script.md) was written ~2026-07-31 and predates a lot.
**New, must appear** in B/C/D:

- **Collections** (V10) — share a mod-set as a *link* (secret gist or public repo), with an import
  preview that tells you what you'd need. The "here's my co-op pack / the server's list" story.
  Missing entirely from the old script.
- **Update by drop → Adopt** (V8) — the real ModHub update flow: drop the new zip, Silo says
  "newer build — Adopt," done. Very demo-able; add to the "keeps mods current" beat.
- **Tile UI + green active switch** (V3) — the old script says "click the dot." Visuals changed;
  re-shoot those beats.
- **Library health / foreign files**, **Find in Library ↔ Browse**, **non-modal drawers**,
  **Needs-update filter** (V13) — newer; good B-roll and small beats, not headlines.
- **Cloud-folder handling** — *not* a highlight (it's a gotcha-handler). One honest line only, in
  C's "careful with your files" beat: on OneDrive/Drive/Proton folders Silo uses copies instead of
  links; still safe, just more disk.

**Still the stars (unchanged):** crash diagnosis, provenance/verify (the moat), conflicts, loadouts,
one-catalog browse, and the rapid-fire tools.

---

## The Tier-D deep-dive series (YT, talking head + in-game)

Split by **job-to-be-done** for retention. Full beat sheets in
[`video-deep-dives.md`](./video-deep-dives.md). Record **Ep 1 first** — it's the hero and the most
searched pain.

1. **"Fix an FS25 crash in 2 minutes"** — diagnose + bisection + conflicts. (V1, V2, V5)
2. **"Manage 700 mods without the chaos"** — library, organize, active/parked, loadouts,
   collections, updates + adopt. (V3, V4, V8, V9, V10, V13)
3. **"Is that mod actually safe?"** — provenance/integrity + the trust story. The differentiator.
   (V6)
4. **"Co-op without the version roulette"** — multiplayer sync + collections. (V11, V10)
5. *(optional)* **"The weird fixes"** — filltype bridge, bindings, savegame editing. (V12)

Each D episode shares a **cold open**, a **trust beat**, and a **CTA** block (in the deep-dives doc)
so they feel like a series.

---

## Cross-references

- **Positioning, voice, trust facts, guardrails:** [`README.md`](./README.md) §1.
- **YouTube channel structure + Shorts pipeline:** [`youtube.md`](./youtube.md).
- **Short-form repurposing (A → TikTok/Reels/X):** [`tiktok-instagram.md`](./tiktok-instagram.md).
- **Feature ground-truth for scripting:** the app's own [`/help`](../../landing/help/index.html)
  page — it's now current, so use it to check exact button names and flows.

## Hard rules (from README, restated because they matter on camera)

- **Show, then say.** Do the thing on screen first, then explain it.
- **No synthetic avatar / AI VO / AI b-roll** on hero content. AI is fine for invisible production
  (script, captions, silence-trim, thumbnail).
- **Own the AI part** honestly, once, near the end — don't hide it, don't lead with it.
- **Never overclaim.** "Provenance, not antivirus." "Unverified ≠ bad." Nothing you can't demo.
- **No personal handles on screen** — close Settings/account panels before recording.
