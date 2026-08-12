<!-- Generated launch plan. Part of Silo's social strategy — see README.md. -->

# Silo on YouTube

> A YouTube channel plan for Silo: a 60-90s hero demo, three tutorial Shorts, a devlog/changelog series, FS25 SEO, and a Shorts pipeline that feeds TikTok/Reels — plus the Claude skills to run it semi-autonomously.

## YouTube

YouTube is Silo's **demo stage and evergreen search shelf**. Silo's whole pitch is visual and hard to believe in text ("it names the mod that crashed you"; "it verifies a mod is the real, untampered build"). Fifteen seconds of the app parsing `log.txt` and printing the culprit's name does more than a paragraph ever will. Two things make YouTube uniquely valuable for us:

1. **It's a search engine, not just a feed.** FS25 players type "farming simulator 25 which mod is crashing" and "fs25 mod conflict" into YouTube constantly. A well-titled video keeps earning views for a year. TikTok/Reels forget you in 48 hours; YouTube compounds.
2. **It's the canonical embed.** The hero demo lives on the landing page (`silo.hllmr.com`), in the GitHub README, in Reddit/Discord replies, and in the ModHub description. One asset, linked everywhere.

**Who we reach here:** the middle and long tail of the FS25 audience — players who watch "top 10 FS25 mods" and troubleshooting videos, plus mod authors who'll care about provenance/verification. Skews slightly more patient and older than TikTok; many arrive via search with a problem already in mind.

**Voice for YouTube:** honest, calm, competent — a builder showing you the thing, not a hype channel. Screen-recording first, face optional. Lead with the pain in the first 3 seconds ("Your game crashed. The log is 4,000 lines. Which mod did it?"), then show the fix. Witty in the script, never clickbait in a way the video can't cash. Long-form devlogs can be more personal/nerdy; the demo and tutorials stay tight and useful. Always disclose: this is the solo dev, it's free and open source, no account, no telemetry.

---

### Why this platform fits Silo

- Every killer feature is a **visual before/after**: 4,000-line log → one mod name; "Verified" vs "Modified (these 3 files changed)"; three sources → one catalog with the newest version highlighted; a reversible on-disk change shown being undone.
- **Evergreen SEO** matches Silo's evergreen value — troubleshooting a crash is a forever-problem for FS25.
- YouTube is the **source of truth for the Shorts pipeline**: cut once, the same vertical clips seed TikTok and Reels (see the pipeline skill below), so YouTube pays for itself twice.
- Devlogs build the **provenance-as-moat narrative** honestly and in public — exactly the trust story a "verify the real build" product needs.

---

### Account / channel SETUP

**Handle:** `@SiloModManager` (primary). Fallbacks if taken: `@GetSilo`, `@SiloFS25`, `@HLLMRSilo`. Keep it identical to the handle used on TikTok/Reddit/Discord so cross-links are obvious.

**Channel name:** `Silo — FS25 Mod Manager`.

**Setup checklist:**
- Create as a **Brand Account** (not a personal Google identity) so it can be co-managed later without exposing a personal inbox.
- **Avatar:** the Silo grain-silo mark on the "Golden Hour" green (`--green-600 #4a7330`). Same file as every other platform.
- **Banner:** tagline "The FS25 mod manager that should have shipped with the game." + `silo.hllmr.com` + "Free · Open source · No telemetry." Safe-area-aware (readable on mobile crop).
- **Channel description** (SEO-loaded, first 2 lines matter most): "Silo is a free, open-source mod manager for Farming Simulator 25. It names the mod that crashed you, catches conflicts before launch, verifies a mod is the real untampered build, and pulls ModHub + GitHub into one catalog. Windows-first, macOS/Linux experimental. No account, no telemetry." Then links: Download, GitHub, Discord, landing.
- **Links:** `silo.hllmr.com`, `github.com/HLLMR/silo`, Discord invite, `silo.hllmr.com/browse`.
- **Channel keywords/tags:** farming simulator 25, fs25, mods, mod manager, fs25 mods, mod conflict, game crash, modhub.
- **Featured/trailer:** set the 60-90s hero demo as the **channel trailer for non-subscribers** and the featured video for returning viewers.
- **Handles verification:** claim the same handle on `youtube.com/@SiloModManager`; add the channel URL to the landing page footer and GitHub README.
- **Sections on the channel homepage** (see STRUCTURE).
- **Community tab** unlocks at 500 subs — plan to use it for changelog polls and screenshots once eligible; until then, pinned comments do that job.

---

### STRUCTURE (channel layout)

Organize the homepage into **shelves/playlists** so a first-time visitor instantly sees the three jobs Silo does:

- **Channel trailer:** `Silo in 90 seconds` (the hero demo) — shown to non-subscribers.
- **Playlist: "Start here"** — hero demo + the three tutorial Shorts. This is the onboarding funnel.
- **Playlist: "Fix my game (troubleshooting)"** — "Which mod crashed me?", "Catch a conflict before launch", guided bisection, MP sync verify. Pure SEO-magnet how-tos.
- **Playlist: "One catalog"** — browse/search across ModHub+GitHub, update-all, provenance/verify a mod. Feature tours.
- **Playlist: "Devlog"** — the running behind-the-scenes/changelog series.
- **Playlist: "Shorts"** — auto-populated vertical clips.
- **Pinned comment convention:** every video's pinned comment = one-line download link + "free & open source, I'm the solo dev, ask me anything." (This is our honesty + support surface pre-Community-tab.)

---

### Content PILLARS

1. **The Fix (troubleshooting SEO)** — "your game is broken, here's the 20-second fix." Highest search value. Crash triage, conflict detection, bisection, MP sync. Titled for how players actually search.
2. **The Tour (feature demos)** — calm walkthroughs of one feature each: browse/catalog, update-all, provenance/verify, filltype bridge, profiles/loadouts. Evergreen "what is Silo / how do I…".
3. **The Build (devlog/changelog)** — a per-release series: what shipped, why, what's next, shown honestly in-app. Builds trust + the provenance-moat story.
4. **The Short (hooks)** — 15-40s vertical clips, one jaw-drop each, engineered to be cut from pillars 1-2 and re-used on TikTok/Reels.

---

### Posting CADENCE (realistic for a solo dev)

- **Shorts: 2-3 per week.** These are cheap because they're cut from existing long-form/demo footage by the clip skill — not filmed fresh.
- **Long-form: 1 every 1-2 weeks.** Alternate a "Fix" how-to and a "Tour" demo.
- **Devlog: 1 per GitHub release** (Silo ships often; cap at ~2/month so it stays watchable — batch small releases).
- Never post filler to hit a number. A dead week is fine; a bad video hurts the SEO shelf.

---

### First-week LAUNCH content (concrete)

**Day 0 — Hero demo (pillar 2, the anchor).**
- Title: `Silo — the FS25 mod manager that should have shipped with the game`
- 60-90s, no talking-head required. Beat sheet: (0-3s) "700 mods. Which one just crashed you?" (3-15s) drop `log.txt` in → culprit named. (15-30s) conflict caught before launch. (30-45s) three sources → one catalog, newest version glowing. (45-60s) "Verified ✓" provenance badge. (60-75s) "Everything's reversible. No account. No telemetry. Free and open source." → download CTA.
- Set as channel trailer. Embed on landing + README same day.

**Day 1 — Short: "Which mod crashed you?"** (pillar 4, cut from the demo)
- Hook (0-2s): "Your FS25 log is 4,000 lines. Silo reads it for you." Show the one-line culprit. End card → full demo. Ship the same file to TikTok/Reels.

**Day 2 — Tutorial Short: "Verify a mod is the real build."**
- Hook: "Did you download the *real* mod, or a tampered one?" Show hash → **Verified** vs **Modified (these files changed)**. This is the moat, in 30 seconds.

**Day 3 — Long-form "Fix": `FS25 keeps crashing? Find the exact mod in 30 seconds`**
- The SEO workhorse. Real crash, real `log.txt`, diagnose → name → (if log can't name it) guided bisection. Title/desc packed with the phrases players search.

**Day 4 — Tutorial Short: "One catalog for ModHub + GitHub."**
- Hook: "Stop checking three sites for one mod update." Search once, see newest-across-sources, update-all.

**Day 5 — Devlog #1: `Why I built Silo (and what "verified" actually means)`**
- 3-5 min, personal, honest. The origin, the provenance moat, the roadmap, "it's free and I'm one person." Invites the first Discord/GitHub community in.

**Day 6-7 — Short: "Everything Silo changes is reversible."**
- Hook: "Scared to let an app touch your mods folder? Watch me undo all of it." Show the projection + undo. Addresses the #1 trust objection.

---

### Thumbnails & titles (system, not one-offs)

- **Thumbnail template:** dark `--green-900`/soil background, ONE big before/after visual (e.g. red crash log ↔ green mod name), 3-4 huge words max ("WHICH MOD?", "VERIFIED ✓", "3 SITES → 1"), the Silo mark bottom-corner. Consistent = recognizable in search. Fraunces display face to match the brand.
- **Title formula for "Fix" videos:** `[Problem the way players type it] + [Silo does it] + [timeframe]` → "FS25 Crashing on Startup? Find the Mod That Did It in 30 Seconds". Front-load the searched keyword.
- **Title formula for "Tour" videos:** "How to [job] in FS25 — Silo".
- **Never** promise in the thumbnail what the first 15 seconds can't deliver. Clickbait tanks watch-time and our SEO.

---

### SEO for FS25 search

- **Keyword targets** (validate/refresh via the SEO skill): "fs25 mod manager", "farming simulator 25 crash fix", "fs25 which mod is crashing", "fs25 mod conflict", "fs25 mods not loading", "farming simulator 25 modhub vs github", "fs25 update mods".
- **Description template:** first 2 lines = the value + primary keyword + download link (this is what shows in search snippets). Then timestamped chapters, then links, then a short honest boilerplate.
- **Chapters** on every long-form (boosts key-moment surfacing).
- **Pinned comment** with the download link on every video (drives conversion + engagement signal).
- **End screens + cards** always route to the hero demo or the relevant "Fix" video (keeps session time on-channel).
- **File the source clip once, title it for search** — the clip skill inherits the SEO metadata into the Shorts.

---

### Shorts pipeline feeding TikTok/Reels

The pipeline is the efficiency multiplier: **film/record long-form once, harvest many verticals.**

1. Record the demo and tour footage at a resolution that crops cleanly to 9:16 (keep the action in a center-safe column, or record key beats twice — once framed for vertical).
2. The **clip-cutter skill** (below) proposes 15-40s moments with the biggest "wow" delta and cuts vertical drafts with burned-in captions (sound-off viewing is the default).
3. A human approves the cut and the hook line.
4. The **cross-post packager** exports one master vertical + platform-specific metadata (YouTube Shorts desc with #Shorts, a TikTok caption, a Reels caption) so the same asset seeds all three from one approval — **drafted for a human to post, never auto-published, never mass-anything.**
5. YouTube Short links back to the long-form; TikTok/Reels link to the landing page.

---

### Claude skills to build

Automations a Claude Code "Cowork" agent builds to run the channel semi-autonomously. **Every one drafts-and-queues for a human to approve; none auto-publishes, DMs, or games metrics.**

**1. `silo-devlog-from-release`**
- **Does:** Turns a shipped GitHub release into a devlog video *package*: a script/beat-sheet (what shipped, why it matters to players, honest caveats), an on-screen shot list mapped to the changelog items, a title + SEO description + chapters, and thumbnail copy. Flags features that are demo-worthy vs. text-only.
- **Trigger:** on new GitHub release (`HLLMR/silo` tag published).
- **Inputs:** release notes + diff summary, `CLAUDE.md` feature list, prior devlog scripts (for voice), keyword list.
- **Outputs:** `devlog-draft-vX.Y.Z.md` (script + shot list + metadata) queued in a review folder; a Discord/DM ping to the dev that a draft is ready. No upload.

**2. `silo-clip-cutter`**
- **Does:** Ingests a long-form recording (+ transcript), proposes the top N 15-40s "wow" moments ranked by before/after delta, and produces vertical (9:16) draft cuts with burned-in captions and a suggested hook line per clip.
- **Trigger:** on new long-form footage dropped in the `to-clip/` folder (or manual).
- **Inputs:** video file + transcript/timestamps, hook-line style guide, brand caption style.
- **Outputs:** N vertical clip drafts + a `clips-manifest.md` (timestamp, hook, suggested title) for human approval. Cutting via a local ffmpeg step the skill scripts; no publish.

**3. `silo-shorts-packager` (the cross-post multiplier)**
- **Does:** Takes ONE approved vertical clip and emits platform-ready metadata bundles for YouTube Shorts, TikTok, and Reels (captions, hashtags, hook, link) so a human posts the same asset to all three in minutes. Enforces per-platform norms (e.g. `#Shorts`, length limits) and the honesty boilerplate.
- **Trigger:** on human approving a clip (moved to `approved/`).
- **Inputs:** approved clip + its manifest entry, per-platform templates, current campaign links.
- **Outputs:** `packages/<clip>/youtube.txt|tiktok.txt|reels.txt` + a checklist. Human uploads; the skill never touches the accounts.

**4. `silo-title-thumb-optimizer`**
- **Does:** For a queued video, generates 3-5 title variants (search-keyword-front-loaded) and matching thumbnail-copy directions against the brand template, with a note on which keyword each targets and the honesty check ("does the first 15s deliver this?").
- **Trigger:** on a video entering the publish queue; or weekly on the backlog.
- **Inputs:** video topic + transcript, keyword list, thumbnail template spec, past CTR data (once available).
- **Outputs:** `title-thumb-options-<video>.md` for human pick. No metadata is pushed live automatically.

**5. `silo-seo-keyword-refresh`**
- **Does:** Refreshes the FS25 keyword/target list by pulling YouTube autocomplete + related-search signals and current FS25 trends, flags rising queries (e.g. a new patch causing crashes = a timely "Fix" video), and proposes 2-3 concrete video ideas.
- **Trigger:** weekly; and on major FS25 game patch detected.
- **Inputs:** current keyword list, autocomplete/search results (WebSearch/WebFetch), recent channel performance.
- **Outputs:** updated `keywords.md` + a ranked `video-ideas.md`. Advisory only.

**6. `silo-comment-triage`**
- **Does:** Reads new comments, classifies them (support question / bug report / feature idea / praise / spam-or-troll), drafts honest, helpful reply suggestions for real questions, and routes genuine bug reports to a GitHub issue draft. Never posts on its own; never engages trolls.
- **Trigger:** daily digest (or on-demand).
- **Inputs:** recent comments (read-only), FAQ/docs, known-issues list.
- **Outputs:** `comment-digest.md` with suggested replies (human sends), + draft GitHub issues for confirmed bugs. Flags anything needing a human judgment call.

**7. `silo-launch-scheduler`**
- **Does:** Maintains the content calendar — sequences the first-week launch plan and ongoing cadence, tells the human what to record/publish next, and nudges when a GitHub release means a devlog is due. A planning assistant, not an auto-poster.
- **Trigger:** weekly; and on new-release event.
- **Inputs:** the cadence rules above, release feed, current draft/approved queue state.
- **Outputs:** `content-calendar.md` updated + a "this week: do X, Y" summary ping.

---

### METRICS that matter (in priority order)

1. **Average view duration / % viewed** on the hero demo and "Fix" videos — the real signal that the demo lands and the SEO videos deserve to rank.
2. **Impressions → CTR** per thumbnail/title (feeds the optimizer skill).
3. **Traffic source = YouTube Search** growth — proof the evergreen SEO shelf is working (the compounding asset).
4. **Click-through to `silo.hllmr.com` / downloads** (via UTM links in descriptions) — the only metric that maps to actual product adoption.
5. **Subscribers** — lagging/vanity; watch the trend, don't optimize for it.
6. Shorts: **views + swipe-through**, but treat Shorts as top-of-funnel; conversion happens on long-form and the site.

### Guardrails

- **No sub-for-sub, no bought views/comments, no fake engagement — ever.** It also poisons the SEO signal we actually want.
- **Disclose:** solo dev, free, open source, no telemetry — in the boilerplate and pinned comment. Disclose AI-assisted content where norms expect it (e.g. if a voiceover is synthetic, say so).
- **Never clickbait past what the first 15 seconds delivers** — YouTube punishes it via watch-time and it burns trust with a trust-product.
- **Respect other creators/mods:** when showing third-party mods in demos, don't disparage authors; provenance shows facts ("these files differ"), never accusations.
- **Comment moderation stays human** for anything sensitive; the triage skill never posts autonomously.
- **Copyright/music:** only licensed/royalty-free audio; no ripped FS25 OST.

### Effort / priority

- **P0 (launch-critical, week 1):** hero demo + channel setup + `silo-clip-cutter` + `silo-shorts-packager` (the demo and the pipeline are the whole reason to be here).
- **P1 (weeks 2-4):** the three tutorial Shorts, first "Fix" long-form, Devlog #1, `silo-devlog-from-release`, `silo-seo-keyword-refresh`.
- **P2 (month 2+):** `silo-title-thumb-optimizer`, `silo-comment-triage`, `silo-launch-scheduler`, Community-tab usage once past 500 subs.
- **Ongoing cost is low** because the clip/packager/devlog skills turn each recording session and each release into multiple assets — the pipeline is the point.

---
[← Back to the social strategy index](./README.md)
