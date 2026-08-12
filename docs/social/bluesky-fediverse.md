<!-- Generated launch plan. Part of Silo's social strategy — see README.md. -->

# Silo on Bluesky + Mastodon

> A lightweight, honest build-in-public presence for Silo on Bluesky and Mastodon that wins the FOSS/indie/gamedev crowd with the provenance + "which mod crashed you" story, fed by AI-assisted-but-human-posted changelog, cross-post, and reply-triage skills.

## Bluesky + Mastodon (fediverse)

### Why this platform fits Silo

Silo is a free, open-source, no-telemetry, everything-is-reversible desktop app — and the fediverse is the one place where those adjectives are the *headline*, not the fine print. The audience here is not the casual FS25 player in a Facebook group; it's the OSS/indie/gamedev crowd, tech-adjacent early adopters, and the security-minded folks who immediately grasp why a **cross-source canonical-hash provenance DB** ("verify a mod is the real, untampered build") is a big deal. This is the platform to:

- Plant the **credibility flag**: "real OSS, no account, no telemetry, source on GitHub" lands hard here and gets boosted by people who care.
- Reach **mod authors and tooling-minded FS25 players** (the tech-savvy minority the brief calls out — the ones on GitHub/Reddit, not Facebook).
- Recruit **contributors, testers, and translators** — build-in-public is native culture on `#indiedev` / `#gamedev` / `#FOSS`.
- Own our narrative on a platform with **no link suppression** (unlike X, Bluesky and Mastodon don't nerf posts that contain outbound links — critical for a project whose CTA is "download the app / read the docs").

This is a **lightweight, high-signal** channel, not a volume play. Realistic expectation: modest follower counts but disproportionately high-quality engagement (GitHub stars, issues, PRs, boosts from respected accounts, press pickup).

### Who we reach here

- **FOSS people** (Fosstodon, `#FOSS`, `#OpenSource`) — value the ethos, amplify the provenance/no-telemetry story.
- **Gamedev / indie devs** (`mastodon.gamedev.place`, `#gamedev`, `#indiedev`) — appreciate the Tauri/Rust/Svelte build-in-public and the "tool the game should have shipped with" framing.
- **Security / supply-chain folks** — the provenance moat is genuinely novel; this is the crowd that says "wait, you hash mods against canonical source builds? tell me more."
- **Tech-savvy FS25 modders & authors** — the ones who'll file good bug reports and maybe contribute.

### Voice for THIS platform

Honest, technical, generous, a little witty — **peer-to-peer, not marketing**. Fediverse culture punishes salesy broadcast and rewards showing your work, crediting others, and being a real person. Lean into the pain ("which mod crashed me?") but back it with substance (a real log line, a real diff, a real screenshot). Dial the Gen-Z energy *down* here vs. TikTok/Discord — dry wit over hype. Always disclose it's the dev/maintainer. Always add alt text (non-negotiable fediverse norm). Boost and reply to others more than you self-promote.

---

### Account / space SETUP

#### Bluesky

- **Handle:** set the account's handle to the domain `@silo.hllmr.com` (Bluesky lets you verify ownership by DNS TXT record). This is free, self-serve verification and instantly signals "this is the official project account" — do this on day one. Keep a fallback `@silohllmr.bsky.social` reserved.
- **Display name:** `Silo — FS25 mod manager`
- **Avatar:** the Silo mark (grain-silo glyph) on the Golden Hour green (`#4a7330`). **Banner:** the app's library/diagnose screen with the one-liner "Names the mod that crashed you."
- **Bio (≤256 chars):** "Free, open-source mod manager for Farming Simulator 25. Names the mod that crashed you, catches conflicts, verifies a mod is the real build. No account, no telemetry, all reversible. ⛓ silo.hllmr.com · </> github.com/HLLMR/silo"
- **Pinned post:** a 4-post launch thread (see launch content) with a 20-30s screen-capture of `◆ diagnose` naming a culprit mod.
- **Starter Pack:** create a "FS25 / farming-sim modding" Starter Pack seeded with real FS25 modders, tool authors, and FOSS-gaming accounts you already follow — a genuine curation, not a follower trap. Great low-effort discovery surface.
- **Custom feed (optional, later):** a `#FS25` feed via a feed generator, if the tag gets traction. Not launch-critical.

#### Mastodon

- **Home instance:** **`fosstodon.org`** as primary (FOSS-native, high-credibility audience, exactly our ethos). Note: Fosstodon requires **manual approval** and is **strict about unattended crossposting/marketing bots** — apply early (a week+ lead time), describe Silo honestly as an OSS project account run by a human maintainer, and commit to native, human-in-the-loop posting. Handle target: `@silo@fosstodon.org`.
- **Also register** `@silo@mastodon.gamedev.place` (bot-and-crosspost-friendlier, gamedev reach) — this is the instance to point any *automated* posting at, keeping Fosstodon clean/native. If you only want one account, keep Fosstodon and post natively.
- **Profile:**
  - Display name `Silo · FS25 mod manager`, same avatar/banner as Bluesky.
  - **Profile metadata fields** (Mastodon shows up to 4, and a link back verifies with a green check via `rel="me"`): `Website → silo.hllmr.com` (add `<link rel="me" href="https://fosstodon.org/@silo">` on the landing page to get the ✅ verification), `Source → github.com/HLLMR/silo`, `Docs → silo.hllmr.com/help`, `Built with → Tauri · Rust · Svelte`.
  - Bio: same value prop as Bluesky, plus `#FOSS #gamedev #FS25`. On Mastodon, hashtags in the bio are followable — include the 2-3 you want to own.
- **Pinned posts (Mastodon allows up to 5):** (1) the launch thread, (2) an "About Silo / what it does" evergreen post, (3) the provenance explainer, (4) "How to try the beta" with the download + `#FS25` tag.
- **Bridging (optional):** enable **Bridgy Fed** so Bluesky followers can follow the Mastodon account (and vice-versa) across protocols — one-time opt-in, extends reach without extra posting. Verify each instance permits it.

---

### STRUCTURE (profile / pinned / highlights)

Because these are microblogs, "structure" = the pinned + evergreen scaffolding a new visitor hits:

| Slot | Bluesky | Mastodon |
|------|---------|----------|
| Handle/verification | `@silo.hllmr.com` (DNS) | `rel="me"` ✅ to silo.hllmr.com |
| Pinned #1 | Launch thread w/ diagnose clip | Launch thread |
| Pinned #2 | (Bluesky pins 1) → use thread | "What Silo is" evergreen |
| Pinned #3 | — | Provenance explainer |
| Pinned #4 | — | "Try the beta" + download |
| Starter Pack | FS25/FOSS-gaming curation | (n/a) |
| Followed tags | `#FS25 #gamedev` | `#FS25 #FOSS #gamedev #indiedev` in bio |

Evergreen posts to keep pinned/ready: **What it is**, **Provenance explainer**, **No-telemetry/reversible promise**, **How to try the beta**.

---

### Content PILLARS

1. **"Which mod crashed you?"** (the hook) — real `◆ diagnose` output naming a culprit, guided-bisection clips, before/after of a fixed load. Highest-shareability pillar.
2. **Build-in-public / changelog** — every release, plus dev-diary snippets ("shipped cross-mod fillType override detection today, here's the gnarly last-wins bug it catches"). Native to `#indiedev`.
3. **Provenance & trust (the moat)** — "Verified / Modified / Unverified," supply-chain framing, "no other tool holds a cross-source canonical-hash DB." The pillar that earns respect from FOSS/security folks.
4. **FS25 modding utility** — genuinely helpful tips (conflict types, the "sugar beet" fillType bridge, MP manifest sync) that stand alone even if you never install Silo. Generosity buys goodwill.
5. **Ethos & community** — no account, no telemetry, everything reversible, source open; boosting *other* FS/FOSS/gamedev projects and crediting contributors. This is the "good OSS citizen" pillar.

Rough mix per week: 40% pillars 1+4 (hooks + utility), 30% pillar 2 (build-in-public), 15% pillar 3, 15% pillar 5 (boosts/replies/community).

### CADENCE

Quality over volume — the fediverse rewards it and Fosstodon's culture expects it.

- **Bluesky:** 3-5 posts/week + replies. More tolerant of casual/frequent posting.
- **Mastodon:** 2-4 posts/week + boosts/replies. Skew toward substantive.
- **Release cadence:** on every GitHub release, a **changelog thread** (both platforms) — the anchor content.
- **Weekly rhythm (template):**
  - *Mon* — build-in-public / "shipping this week" (pillar 2)
  - *Wed* — utility tip or `diagnose`/conflict demo (pillar 1/4)
  - *Fri* — provenance/ethos or community boost + a "try the beta" nudge (pillar 3/5)
  - *Ad hoc* — release threads, replies to mentions, joining relevant `#FS25`/`#gamedev` convos.
- **Golden rule:** reply/boost more than you broadcast. At least 1:1 community-interaction to self-post.

### First-week LAUNCH content (concrete)

**Day 1 — Launch thread (pinned, both platforms).** 4 posts:
1. "Farming Sim 25 gives you a flat `mods/` folder and a shrug. Silo is the management layer it forgot to ship. Free, open source, no account, no telemetry. 🧵" + 20-30s clip of the library view.
2. "The headline feature: it reads `log.txt` and **names the mod that actually crashed you** — separating real errors from cosmetic noise. No more disable-half-and-pray." + `◆ diagnose` screenshot naming a culprit.
3. "It also catches conflicts *before* launch (duplicate active map = instant crash, cross-mod fillType last-wins) and pulls ModHub + GitHub into one catalog with the latest version across all three."
4. "And the part I'm proudest of: **provenance.** Silo hashes an installed mod and tells you if it's the real, untampered build — Verified / Modified (with the exact changed files) / Unverified. Windows beta today, macOS/Linux experimental. → silo.hllmr.com · source: github.com/HLLMR/silo"

**Day 2 — "Which mod crashed you?" deep-dive.** A single crash log line most FS25 players recognize, then the `diagnose` output that names it. Ask: "Reply with your worst FS25 crash — bet I can tell you which mod." (genuine engagement, not a growth hack)

**Day 3 — Provenance explainer** (pillar 3, security framing). "Mods get re-uploaded, repackaged, and tampered with all the time. How do you know the file on your disk is the one the author actually built? Silo does byte-exact verification against a canonical hash from source. Here's how ⛓" + diagram/screenshot. This is the post most likely to get boosted by the FOSS crowd.

**Day 4 — Utility, no strings.** The "sugar beet" fillType bridge tip: how a stubborn map fillType won't load into your equipment's categories, and the fix — useful even to non-users. "Silo can generate the companion mod for you, but here's the concept either way."

**Day 5 — Build-in-public.** A real snippet from the repo (a test, a tricky conflict-detection edge case, "63 Rust tests and counting"), Tauri/Rust/Svelte tag bait for `#rustlang #gamedev`. Credit the stack, invite contributors/testers.

**Day 6 — Community boost.** Boost/shout-out a respected FS25 modder or FOSS-gaming tool, no ask attached. Establishes we're a citizen, not a billboard.

**Day 7 — Recap + soft CTA.** "Week one of Silo in the open. If you run FS25 on Windows and want to stop guessing which mod broke your save: silo.hllmr.com. Bugs → GitHub issues. Thanks for the boosts." Pin/refresh the thread.

---

### Claude skills to build

Automations a Cowork agent builds so this channel runs semi-autonomously. **Every one drafts/queues for a human to approve and post** — nothing here auto-publishes to Fosstodon (against its norms) or mass-interacts. Automated *publishing*, where used at all, targets only the bot-friendly Mastodon instance and Bluesky, and is clearly a project account.

- **`changelog-to-fedi`**
  - *What it does:* Converts a GitHub release / `CHANGELOG.md` entry into a platform-adapted **draft** — a Bluesky thread (≤300 chars/post, native links, no "link in bio") and a Mastodon post (≤500 chars, followable hashtags, alt-text placeholders). Picks the 2-3 user-facing highlights, drops internal refactors, appends the download + repo links.
  - *Trigger:* on new GitHub release (release-published webhook / `gh` poll).
  - *Inputs:* release tag, body, `CHANGELOG.md`, links. *Outputs:* two ready-to-edit draft files in a review queue + a Slack/Discord ping to the maintainer.

- **`x-to-fedi-adapter`**
  - *What it does:* Takes an X post/thread (the cross-post source of truth) and rewrites it for each platform — strips X-isms, re-lengths for Bluesky vs. Mastodon, swaps in platform-appropriate hashtags, flags any image lacking alt text, and *never* mirrors verbatim (avoids the "obvious crosspost bot" smell).
  - *Trigger:* on new X post tagged for syndication (manual flag or label).
  - *Inputs:* X post text + media. *Outputs:* Bluesky + Mastodon drafts with alt-text prompts, queued for approval.

- **`fedi-content-calendar`**
  - *What it does:* Generates the week's draft calendar from the pillar mix + weekly rhythm template, pulling candidate hooks from recent commits/issues/screenshots so drafts are grounded in real work, not filler. Flags gaps ("no pillar-3 post this week").
  - *Trigger:* weekly (e.g., Sunday).
  - *Inputs:* recent git log, open issues, screenshot/asset folder, pillar targets. *Outputs:* a 3-5-item drafted calendar in the review queue.

- **`draft-and-queue-scheduler`**
  - *What it does:* Takes approved drafts and schedules them via a scheduling backend (Buffer/Fedica or direct AT-Protocol + Mastodon API for the bot-friendly instance), spacing posts and avoiding collisions with release threads. Never posts to Fosstodon unattended.
  - *Trigger:* on draft approval / cron.
  - *Inputs:* approved drafts + desired times. *Outputs:* scheduled posts + a confirmation log; Fosstodon items routed to "post manually" with a reminder.

- **`alt-text-writer`**
  - *What it does:* Writes concise, accurate alt text for every screenshot/clip/still before it's queued (hard fediverse norm; blocks any image without it). Describes what's on screen (e.g., "Silo diagnose panel listing FS25_ExampleMod as the crash culprit").
  - *Trigger:* on image added to a draft.
  - *Inputs:* image (+ optional context caption). *Outputs:* alt-text string attached to the draft; a warning if a draft ships without one.

- **`mention-and-reply-triage`**
  - *What it does:* Pulls mentions/replies/DMs from both platforms, classifies them (bug report / support question / feature ask / praise / press/partnership / troll), drafts a suggested human reply in Silo's voice, and routes real bugs toward a GitHub issue template. Never auto-sends; never mass-DMs.
  - *Trigger:* on new mention/reply (API poll, e.g., every 30-60 min) or manual run.
  - *Inputs:* mention thread context + Silo docs/FAQ. *Outputs:* a triaged inbox with per-item drafted replies + suggested labels/GitHub-issue stubs for the maintainer to approve.

- **`clip-and-still-cutter`**
  - *What it does:* Turns a raw screen recording (a `diagnose` run, a conflict catch, an install progress bar) into short, captioned clips and platform-sized stills, with a suggested hook line per pillar. Removes the biggest friction on "show, don't tell."
  - *Trigger:* on new recording dropped in an assets folder.
  - *Inputs:* raw video/screenshots. *Outputs:* trimmed clips + stills sized for Bluesky/Mastodon + draft caption + alt text (via `alt-text-writer`).

- **`tag-and-feed-listener`** *(assist, human acts)*
  - *What it does:* Watches `#FS25`, `#FarmingSimulator`, `#gamedev`, `#FOSS`, and named FS25/tool accounts across both platforms for genuinely relevant conversations Silo could *helpfully* join, and surfaces a short daily digest. It only recommends — a human writes and posts the reply (no automated interaction, no engagement farming).
  - *Trigger:* daily digest (cron).
  - *Inputs:* tag/feed queries. *Outputs:* a ranked digest of "worth replying to" posts with a one-line why + a suggested angle.

---

### METRICS that matter here

Vanity follower counts are the *least* useful metric on the fediverse. Track, roughly in priority order:

- **Referral traffic** to `silo.hllmr.com` and `/help` from `bsky.app` / Mastodon referrers (analytics), and **GitHub stars/clone spikes** correlated to posts.
- **Downloads / new beta users** in windows following launch posts.
- **Quality engagement:** reposts/boosts (especially from respected accounts), substantive replies, and bug reports/issues that cite fediverse as the source.
- **Contributor funnel:** new issues, PRs, translators, testers attributable to build-in-public posts.
- **Provenance-post resonance** specifically — it's the differentiator; watch whether pillar-3 posts outperform on boosts.
- Secondary: follower growth, profile visits, `rel="me"`/Starter Pack adoption.

### Guardrails

- **No astroturfing, fake accounts, vote/boost manipulation, or bought engagement — ever.** All growth is earned.
- **Respect instance rules.** Fosstodon prohibits unattended marketing/crossposting bots and expects genuine community participation — post there **natively, human-in-the-loop**; route any true automation to the bot-friendly instance and label the account honestly.
- **Alt text on every image** (accessibility norm, and Fosstodon rule). Use content warnings where the instance expects them.
- **Disclose AI assistance** where norms expect it (e.g., note when a graphic/summary is AI-assisted); always be clear this is the maintainer/project account, not a "user."
- **Automation assists a human** — it drafts, schedules, and triages; it never mass-DMs, auto-replies to strangers, or impersonates anyone.
- **Reply/boost ≥ self-promo.** Be a citizen first.

### Effort / priority

- **Priority: Medium.** Lower reach than the FS25 player-mass channels (Facebook/Discord/TikTok), but the **highest-credibility, best-ROI channel for the OSS/provenance story and contributor recruitment** — and cheap to run once the skills exist.
- **Effort: Low-to-medium.** Setup is a few hours (handles, DNS/`rel="me"` verification, profiles, pinned threads, Fosstodon application lead time). Ongoing is ~2-3 hrs/week of human review once `changelog-to-fedi`, `x-to-fedi-adapter`, and `mention-and-reply-triage` are live.
- **Sequence:** Bluesky first (self-serve, no approval gate, higher immediate reach) → Fosstodon application in parallel (approval lag) → build `changelog-to-fedi` + `alt-text-writer` first (highest leverage), then the scheduler and triage skills.

---
[← Back to the social strategy index](./README.md)
