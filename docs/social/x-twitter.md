<!-- Generated launch plan. Part of Silo's social strategy — see README.md. -->

# Silo on X / Twitter

> A lightweight, authentic X/Twitter plan for Silo: a build-in-public devlog cadence plus a launch thread that reaches FS players, gamedev, and OSS crowds, run semi-autonomously by six changelog→post and triage Claude skills.

## X / Twitter

### Why X fits Silo (and who we reach here)

X is the fastest, lowest-friction place to *build in public*. Silo is a solo/small-dev, open-source desktop tool — its story is inherently a devlog: "shipped a thing that names the mod that crashed you," "here's the provenance DB catching a tampered build," "v0.2.3 is out." That kind of short, screenshot-driven progress post is X's native format, and it compounds: a devlog builds a searchable trail and a following that carries each release.

Three distinct audiences overlap here, and Silo can speak to all of them from one account:

- **FS / farmsim players** — a smaller but real slice of #FS25 / #FarmingSimulator posts live on X (clips, mod showcases, "which mod broke my save" venting). This is the *demand* audience: they feel the pain Silo fixes.
- **Gamedev / tooling people** — #gamedev, #indiedev, Tauri/Rust/Svelte circles. They won't play FS25, but they respect the engineering (hardlink projection, cross-language canonical-hash provenance, off-thread Rust) and amplify it.
- **OSS / build-in-public** — #buildinpublic, #opensource. They reward transparency, honest changelogs, and "no account, no telemetry." They become contributors and stars.

X is **not** where the casual/older FS majority lives (that's Facebook groups and Discord). So on X we lean into the *builder* and *tech-curious* half — and treat X as the top-of-funnel signal amplifier that feeds people to the landing page and Discord, not the primary support desk.

### Voice for THIS platform

Honest, dry, a little witty — the "which mod crashed me?" energy. Show, don't announce: a screenshot of the diagnose panel naming the culprit beats a paragraph. Ship-and-tell, not hype. Credit mod authors and the community generously. Zero growth-hacking tone. Gen-Z energy stays *dialed down* here vs. TikTok/Discord — X rewards clarity and a good screenshot over slang. Occasional wit is welcome ("your log.txt is 40,000 lines. Silo read it so you don't have to."), forced memes are not.

---

### Account & profile setup

**Handle** (in priority order — check availability, keep it consistent with GitHub `HLLMR` / `silo.hllmr.com`):
1. `@SiloForFS` — clearest, ties to the audience.
2. `@GetSilo` — clean, app-y, matches "get it" CTA.
3. `@SiloModManager` — descriptive, less catchy.
4. `@HLLMR_dev` — personal/dev identity, if the human wants to post as themselves and feature Silo (a valid alt: build-in-public often does better from a *person* than a brand).

Recommendation: register **`@SiloForFS`** as the product account and, optionally, keep the dev's personal handle for RTs/replies so the "solo dev" story stays human. If choosing one, a **personal-led** account (dev's name in the display name, "building Silo") tends to outperform a faceless brand on X's build-in-public side.

**Display name:** `Silo — FS25 mod manager`

**Bio (≤160 chars):**
> Free, open-source mod manager for Farming Simulator 25. Names the mod that crashed you, catches conflicts, verifies real builds. No account, no telemetry. ⬇️

**Profile fields:**
- **Location:** "Windows-first · macOS/Linux experimental" (playful use of the field) or leave real region.
- **Link:** `silo.hllmr.com`
- **Avatar:** the Silo mark on the "Golden Hour" green (`#4a7330`) — identical to app/site so brand == product.
- **Header image:** one clean hero — the diagnose panel with a real culprit named, or the three-source catalog (ModHub + GitHub + Nexus) with a "latest version" badge. Include the tagline "the mod manager that should have shipped with the game."
- **Verification/pro:** optional; a paid checkmark helps thread reach but is not required for launch.

**Pinned post:** the **launch thread** (below) at launch; after ~2 weeks, swap to the best-performing evergreen — likely the 45-second demo clip ("watch Silo name the mod that crashed a save") or the current release post. Re-pin each major release thread.

### Structure (profile / highlights / pinned)

- **Pinned tweet:** launch thread → later the demo clip / latest release.
- **Highlights** (add the launch thread + best release threads to the profile Highlights tab so new visitors see peaks first).
- **Media tab as a portfolio:** because every devlog post carries a screenshot/clip, the Media grid becomes a visual changelog. Keep shots consistent (same theme, same window chrome) so the grid reads as one product.
- **No separate Community/Space at launch** — a Space (audio) is a *later* play (post-1.0 AMA). Don't fragment early.

---

### Content pillars

1. **Ship-and-tell (devlog)** — "shipped X." A feature, a fix, a refactor. Always a screenshot or clip. ~40% of posts.
2. **Pain → fix** — name the FS25 pain, show Silo solving it. "Game crashed, no error name. Here's Silo pointing at the exact mod." The strongest hooks. ~25%.
3. **Under the hood (engineering)** — provenance canonicalization, hardlink projection vs copy fallback, Rust off-thread hashing, guided bisection. For the gamedev/OSS crowd. ~15%.
4. **Community & credit** — RT/QT mod authors, answer "can Silo do X," shout out contributors, celebrate a PR/star milestone honestly. ~15%.
5. **Meta / build-in-public** — "solo dev, here's the roadmap," "why no telemetry," honest "this is still beta and here's what's rough." ~5%.

### Posting cadence (realistic for a solo dev)

- **3–5 posts/week**, not daily. Quality screenshot > volume.
- **1 devlog/week minimum** (ties to actual commits/releases).
- **Reply/QT daily-ish, 10 min:** search `FS25 crash`, `FS25 mod conflict`, `which mod crashed`, `#FS25`, `#FarmingSimulator25` and helpfully reply where Silo genuinely fits — *help first, link only when asked or clearly relevant.* This is where reach is actually earned.
- **Release day = a thread**, every meaningful version.
- **Best windows for this audience:** weekday evenings + weekend mornings (US/EU farmsim players play on weekends). Schedule accordingly; don't post into the 3am void.

### First-week LAUNCH content (concrete)

**Day 0 — Launch thread (pinned).** 6–7 posts:
1. **Hook:** "Your FS25 crashed. The log is 40,000 lines. It never names the mod. Silo does. 🧵 Free & open source, no account, no telemetry." + 15s clip of diagnose naming the culprit.
2. The catalog: ModHub + GitHub + Nexus in one search, latest version across all three — screenshot.
3. Conflict detection before launch (duplicate active map = instant crash caught) — screenshot.
4. Provenance: "Verified / Modified / Unverified — Silo hashes your installed mod against the real build. Not antivirus. Proof." — screenshot with the changed-files list.
5. "Everything it does on disk is reversible. It projects your active set at launch via junctions; your originals never move." — the trust beat.
6. "Windows-first, macOS/Linux experimental. v0.2.2 beta. Built with Tauri + Rust + Svelte. It's open source: [GitHub]. Try it: silo.hllmr.com" + CTA.
7. "I'm one dev. Bugs happen — file them, I fix fast. Roadmap in the repo." (humanizes, sets expectations).

**Day 1 — The demo clip (standalone, for the QT crowd).** 30–45s screen recording, captioned, of the single best flow: crash → diagnose → culprit named → disable → relaunch. Designed to be QT'd. This becomes the eventual pin.

**Day 2 — Pain post.** "Ever spent an hour disabling mods one by one to find the broken one? Silo's guided bisection does the 'disable half, relaunch' loop for you — crash-safe, snapshots your set." + clip.

**Day 3 — Under the hood (gamedev/OSS reach).** "How Silo verifies a mod is the *real* untampered build: a cross-language-ratified canonical hash manifest, matched against a canonical DB built from source. Here's the format 🧵" — 3-post mini-thread. Tag #rustlang #gamedev #buildinpublic.

**Day 4 — Community.** RT/QT a well-known FS25 mod author or a "which mod broke my game" post with a genuinely helpful reply + offer. Ask a question: "Mod authors — would a 'Verified build' badge on your releases be useful?" (real signal-gathering).

**Day 5 — Roadmap / build-in-public.** "Week one of Silo in public: X stars, Y bugs filed, Z fixed. Next up: [feature]. No telemetry means I only know what you tell me — so tell me what's rough." + screenshot of the GitHub issues/board.

**Day 6–7 — Respond & recap.** Reply to everything, QT the best user reactions, and post a short "thanks + here's what shipped this week" wrap. Roll straight into the weekly cadence.

**Hashtags (use 1–3, not a wall):** `#FS25` `#FarmingSimulator25` for the demand audience; `#buildinpublic` `#opensource` `#rustlang` `#gamedev` for the builder audience. Match tags to the post's pillar — don't spray farmsim tags on an engineering post.

---

### Claude skills to build

Automations a Cowork agent builds so the human runs this channel in ~15 min/day. Every skill **drafts and queues for human review** — nothing auto-posts to the public timeline without approval, and nothing DMs anyone.

**1. `silo-release-to-thread`**
- **What:** Turns a GitHub release / `CHANGELOG` diff into a launch-quality X thread (hook + one post per headline change + CTA), following the pillar voice. Suggests which screenshots to attach and drafts alt-text for each.
- **Trigger:** on new GitHub release published (`HLLMR/silo`).
- **Inputs:** release tag, body, commit range, `CHANGELOG.md`, prior threads (for voice consistency).
- **Outputs:** a ready-to-review thread (Markdown, numbered) + a shot list + alt-text, dropped into a review queue file / draft.

**2. `silo-devlog-drafter`**
- **What:** Scans the week's merged PRs/commits and picks the 1–2 most *show-and-tell-able* changes, drafting standalone devlog posts (hook + body + hashtag set) with a "needs screenshot of X" note.
- **Trigger:** weekly (e.g. Friday) + on-demand.
- **Inputs:** merged PRs/commits since last run, issue closes, the pillar/cadence rubric.
- **Outputs:** 2–3 draft posts ranked by expected engagement, queued for review.

**3. `silo-post-scheduler`**
- **What:** Takes approved drafts and schedules them into the best time slots (weekday eve / weekend AM), spacing posts so the account never bursts or goes silent. Maintains a visible content calendar.
- **Trigger:** on draft approval + a daily check.
- **Inputs:** approved-drafts queue, posting-window config, last-posted timestamps.
- **Outputs:** a scheduled queue (calendar file) + a schedule; posts via an X API integration **or**, if no API, emits a copy-paste-ready ordered checklist with times for the human. Never posts unapproved content.

**4. `silo-mention-triage`**
- **What:** Pulls mentions/replies and keyword searches (`FS25 crash`, `which mod crashed`, `mod conflict`, brand mentions), classifies each (bug report / support Q / praise / feature ask / troll), and drafts a helpful, on-voice reply — flagging anything that should become a GitHub issue. **Suggests, never sends; never DMs.**
- **Trigger:** daily (or 2x/day around launch).
- **Inputs:** mentions timeline, saved search terms, an FAQ/known-issues doc, GitHub issue list.
- **Outputs:** a triage digest (item → category → suggested reply → "file issue? y/n"), plus draft GitHub issues for real bugs.

**5. `silo-clip-cutter`**
- **What:** From a raw screen recording of the app, identifies the tight moment (crash → culprit named), trims to a 15–45s captioned clip sized for X, and generates 2–3 caption/hook options. Also crops consistent still screenshots for the Media grid.
- **Trigger:** on-demand (human drops a recording in a watch folder).
- **Inputs:** raw video/screenshots, the flow to feature, brand caption style.
- **Outputs:** trimmed captioned clip + still frames + caption options, ready to attach.

**6. `silo-weekly-recap`**
- **What:** Compiles the honest build-in-public recap — stars/issues opened/closed/PRs merged this week and what shipped — into a short post + a screenshot suggestion of the issues board. Keeps the numbers truthful (pulls real GitHub stats).
- **Trigger:** weekly.
- **Inputs:** GitHub repo stats (stars, issues, PRs, releases) over the window, prior recap for continuity.
- **Outputs:** one draft recap post + supporting stat snapshot, queued for review.

> Shared guardrail baked into all six: outputs are **drafts in a review queue**. A human approves before anything reaches the public timeline. No mass-DM, no auto-follow, no vote/engagement manipulation, no fake accounts. AI-assisted drafting is disclosed where norms expect it, and the account is transparently "the dev / the Silo community."

---

### Metrics that matter (and ones that don't)

**Track:**
- **Landing-page clicks & GitHub referrals from X** (the real conversion — use a UTM on the bio/thread link).
- **GitHub stars & new issues attributable to launch windows** (X's actual job here is funnel + contributors).
- **Thread completion / clip watch-through** (are people getting to the CTA post?).
- **Reply quality** — genuine "does it do X?" / "trying it now" replies, and mod-author engagement.
- **Saves/bookmarks** on how-to and under-the-hood posts (intent signal that outlasts likes).

**Ignore / don't chase:** raw follower count, vanity impressions, like counts on hype posts. A 200-follower account that sends 40 people to the repo is winning.

### Guardrails

- **Platform rules:** no automation that mass-posts, auto-follows, auto-DMs, or manipulates engagement (X ToS + basic decency). Automations draft; a human posts/approves.
- **Honesty:** always "beta," always "solo/small dev," always "Windows-first, mac/Linux experimental." Never overstate provenance as antivirus. Correct mistakes in public.
- **Community respect:** credit mod authors, don't punch down at other tools, help-before-link in replies. Disclose AI-assisted content where the norm expects it.
- **Reversibility as a trust theme** — keep repeating that Silo never destroys game files; it's a differentiator *and* the honest truth.

### Effort / priority

- **P0 (launch week, ~1 day setup + 1 day content):** profile + pinned launch thread + demo clip. Build **`silo-release-to-thread`** and **`silo-mention-triage`** first — they carry the launch.
- **P1 (week 2–3):** **`silo-post-scheduler`** + **`silo-devlog-drafter`** to make the weekly cadence sustainable solo.
- **P2 (month 2):** **`silo-clip-cutter`** and **`silo-weekly-recap`** to lift quality once the rhythm is proven.
- **Ongoing effort:** ~15 min/day (reply/triage from the digest) + ~1–2 hrs on release days. The skills exist specifically to keep it at that level.

---
[← Back to the social strategy index](./README.md)
