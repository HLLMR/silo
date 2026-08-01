# Silo — social launch plans

Launch and community plans for **Silo** (the free, open-source FS25 mod manager) across the
major social platforms. Each platform has its own buildable plan; this page is the strategy
overview and the master list of automation ("Claude skills") to build.

These are **plans to hand to a builder** — concrete channel structures, cadences, content hooks,
and the automations that let a small dev run a real presence without burning out. Every plan is
bound by the same guardrails: **no astroturfing, no fake accounts, no vote-buying, no spam**;
respect each platform's rules; automation drafts and schedules for a human, it never impersonates.

## Platform plans

| Platform | In one line | Plan |
|---|---|---|
| **Discord** | A full buildable blueprint for Silo's Discord community hub — channel/role tree, verification onboarding, bot stack, GitHub-linked support flow, and a first-week activation plan. | [discord.md](./discord.md) |
| **Reddit** | A self-promo-rule-compliant Reddit launch plan for Silo: participate first, lead with the "which mod crashed you?" hook, run a launch post + AMA in r/farmingsimulator and adjacents, and automate changelog-to-post drafting and comment triage with Cowork. | [reddit.md](./reddit.md) |
| **YouTube** | A YouTube channel plan for Silo: a 60-90s hero demo, three tutorial Shorts, a devlog/changelog series, FS25 SEO, and a Shorts pipeline that feeds TikTok/Reels — plus the Claude skills to run it semi-autonomously. | [youtube.md](./youtube.md) |
| **X / Twitter** | A lightweight, authentic X/Twitter plan for Silo: a build-in-public devlog cadence plus a launch thread that reaches FS players, gamedev, and OSS crowds, run semi-autonomously by six changelog→post and triage Claude skills. | [x-twitter.md](./x-twitter.md) |
| **TikTok + Instagram** | A hook-first short-form playbook for Silo on TikTok + IG Reels: repeatable "which mod crashed you" screen-capture formats, a realistic cadence, first-week clips, and the Claude/Cowork skills to run it semi-autonomously. | [tiktok-instagram.md](./tiktok-instagram.md) |
| **Facebook** | A value-first Facebook plan: a helpful Page for updates plus disciplined, rule-respecting group engagement that answers "which mod crashed me?" and converts goodwill into Discord/site traffic without tripping spam filters or group bans. | [facebook-groups.md](./facebook-groups.md) |
| **Bluesky + Mastodon** | A lightweight, honest build-in-public presence for Silo on Bluesky and Mastodon that wins the FOSS/indie/gamedev crowd with the provenance + "which mod crashed you" story, fed by AI-assisted-but-human-posted changelog, cross-post, and reply-triage skills. | [bluesky-fediverse.md](./bluesky-fediverse.md) |
| **GitHub community** | Turn the HLLMR/silo repo into a community funnel — Discussions, templates, good-first-issues, Releases-as-announcements, and a CONTRIBUTING on-ramp — with Claude skills to run it semi-autonomously. | [github-community.md](./github-community.md) |

## Strategy overview

## Launch strategy

Silo's launch problem is not awareness of a category — every FS25 player already lives the pain ("which mod crashed me?"). The job is to show up where that pain gets voiced, prove the fix in one glance, and give people a home to stick around in. This plan reuses one voice across eight channels, nails three of them first, and hands Cowork a deduped set of automation skills so a solo dev can actually sustain it.

### 1. Positioning & voice — one guide, reused everywhere

**One-line positioning:** *The mod manager FS25 should have shipped with — it names the mod that actually crashed you.*

**The three proof pillars** (every piece of content should ladder to one):
1. **"Which mod crashed you?"** — reads `log.txt`, names the culprit, filters cosmetic noise. The hook. Emotionally the strongest; lead with it almost everywhere.
2. **Catch conflicts before launch + one catalog across ModHub/GitHub/Nexus** — the daily-utility story.
3. **Provenance / "is this the real build?"** — the moat and the credibility/trust story. Lead with this for OSS/gamedev/mod-author audiences.

**Always-on trust facts:** free, open source, no account, no telemetry, everything it changes on disk is reversible. These defuse "why should I trust a random exe near my game files" — repeat them, don't bury them.

**Voice guide (the reusable card):**
- **Honest, helpful, a little witty.** We lead with the player's pain, not our feature list. We never overclaim ("provenance, not antivirus").
- **Speak player-first, dev-second.** "You launched, it crashed, the game shrugged" — not "log.txt parser with culprit attribution."
- **Register dial by platform:** dry/clever on Reddit + X; warm/plain-spoken and jargon-free on Facebook Groups (older/casual audience — no "hardlink projection," say "it doesn't move your files"); Gen-Z energy *only* on TikTok/Reels where it's native, never forced; precise/technical on GitHub + Bluesky/Fediverse (provenance-forward).
- **Always disclose we're the dev.** No sock-puppets, no "I found this cool tool." AI-assisted drafts are fine; a human posts and owns every word.
- **Show, don't tell.** The product is visual — a 6-second screen capture of Silo naming a crash beats any paragraph.
- **Banned:** fake urgency, growth-hack cringe, trashing competitors, "revolutionary/game-changer," and any claim we can't demo.

**Visual identity carried across channels:** the "Golden Hour" palette already in the app + landing (green `#4a7330` / gold `#e3b23c` / soil tones), Fraunces display + Hanken Grotesk UI. Every thumbnail, OG image, and channel banner uses it so the product and its marketing read as one thing.

### 2. Sequencing

Silo is **v0.2.2 beta now**; the real event is **1.0**. Treat now→1.0 as build-in-public runway that *earns* the 1.0 spike. Don't blow the launch beats early.

**BEFORE 1.0 (stand up now — foundations + quiet credibility):**
- **Discord server** live and buildable but low-key — it must exist *before* you drive any traffic, or launch clicks bounce off a void. Onboarding/verification, support flow, GitHub-linked channels.
- **GitHub repo as community funnel** — Discussions on, issue/PR templates, CONTRIBUTING, good-first-issues, Releases written as announcements. This is where your most valuable early users (contributors, mod authors) land.
- **Build-in-public devlog cadence** on X + Bluesky/Mastodon — 2–3 posts/week off the changelog. Low volume, honest, builds the audience that will amplify 1.0.
- **Reddit: participate only.** Build comment history and karma in r/farmingsimulator now by *helping* ("here's how to find which mod crashed you"), so you're not a day-one stranger self-promoting into a ban.
- **Capture raw assets continuously:** every crash-triage, conflict-catch, provenance check → screen recording into a clip vault. You'll need them all at launch.

**AT 1.0 (the spike — coordinate a single day/week):**
- **Reddit launch post + AMA** in r/farmingsimulator (and a crosspost-appropriate adjacent), leading with the hook.
- **YouTube 60–90s hero demo** goes live — the canonical "what is this" asset every other channel links to.
- **Launch thread** on X + Bluesky/Mastodon; GitHub 1.0 Release as the announcement of record.
- **First short-form burst** — 3 clips on TikTok/Reels/Shorts from the vault.
- **Discord "we're live" + invite** everywhere points home to Discord and the site.

**AFTER 1.0 (sustain + widen):**
- **Facebook Groups** — enter deliberately *after* you have social proof and a Discord to funnel to. Highest reach for the casual/older majority, but highest ban risk; value-first answers only, no link-dropping.
- **YouTube tutorial Shorts + devlog series**, TikTok/Reels cadence, ongoing changelog→post loop across all channels.
- Turn support wins into content (a real "it named my crash" story is your best ad).

**The 2–3 to nail first, and why:**
1. **Discord** — the owned home base. Retention, support, and the place goodwill from every other channel converts into a community you control. Nothing else works if traffic has nowhere to land.
2. **Reddit (r/farmingsimulator)** — highest-intent discovery for FS25, and the "which mod crashed you?" hook is *native* to how people already post there ("game crashes, no idea why, help"). Best pain-to-audience fit of any channel.
3. **Short-form video (YouTube Shorts as the hub → feeds TikTok/Reels/X)** — the hook is inherently visual; one 6–10s "watch it name the crash" clip is the single highest-leverage asset and repurposes across four platforms from one render. YouTube also owns the evergreen SEO ("FS25 mod crash which mod") and hosts the hero demo everything links to.

*Facebook Groups is the clear #4 (P1) — where the mass casual audience actually is — but it's deferred because it needs social proof + Discord + disciplined humans first. X/Bluesky/Fediverse run continuously as low-cost credibility/devlog, not player-acquisition priorities. GitHub is priority-critical but as a contributor funnel, not a marketing megaphone.*

### 3. Sample 2-week content calendar (priority channels)

Assumes the two weeks *bracketing 1.0*: Week 1 = final pre-launch runway, Week 2 = launch. R = Reddit, D = Discord, YT = YouTube/short-form (auto-repurposed to TikTok/Reels/X/Bluesky), GH = GitHub.

| Day | Reddit | Discord | Short-form / YT | GitHub / Other |
|---|---|---|---|---|
| **Wk1 Mon** | Helpful reply pass in r/farmingsimulator (no promo) | Seed #showcase with 2 dev clips; finalize onboarding | Record + edit hero demo (60–90s) | Cut 1.0 release notes draft |
| **Wk1 Tue** | Answer a "game keeps crashing" thread with the manual method, soft-mention Silo if rules allow | Post devlog: "1.0 is close, here's what's in it" | Clip A: "which mod crashed you?" (10s) — schedule, don't post | X/Bluesky devlog from changelog |
| **Wk1 Wed** | — | Set up AMA channel + pin questions form | Clip B: conflict-caught-before-launch | Good-first-issues labeled, CONTRIBUTING final |
| **Wk1 Thu** | Comment-history building; note 2–3 threads to revisit at launch | Community: "what should 1.0's first patch fix?" poll | Clip C: provenance / "is this the real build?" | Draft GH 1.0 Release-as-announcement |
| **Wk1 Fri** | — | Devlog: behind-the-scenes of the crash triage | Publish hero demo *unlisted*, gather Discord feedback | X/Bluesky "1.0 drops Monday" teaser |
| **Wk1 Sat/Sun** | Light lurking, save threads | Casual presence, answer questions | Buffer/repurpose clips to TikTok/Reels | — |
| **Wk2 Mon (LAUNCH)** | **Launch post** — lead with the hook, honest beta→1.0 story | "We're live" + invite banner, open AMA channel | Hero demo public on YT; Clip A live across TikTok/Reels/Shorts/X | **GH 1.0 Release** published as the announcement of record |
| **Wk2 Tue** | Reply triage on launch post (fast, human, humble) | **AMA day** — dev answers live for a set window | Clip B live | Thank early contributors in Discussions |
| **Wk2 Wed** | Answer the crash threads you saved Wk1, now with a live tool | Post AMA recap | Clip C (provenance) live | Triage inbound issues; convert good ones to good-first-issues |
| **Wk2 Thu** | Respectful crosspost to one adjacent sub if reception is good | Highlight first real user "it named my crash" story | Devlog Short #1: "how it names the crash" | X/Bluesky: share a genuine user win |
| **Wk2 Fri** | — | Ship + announce first fast patch from feedback (huge trust signal) | Patch clip from changelog (auto-drafted) | GH point-release notes |
| **Wk2 Sat/Sun** | Monitor, reply, rest | Community game/screenshot prompt (retention) | Repurpose top clip; queue next week | Begin **first value-first Facebook Group** answers now that proof exists |

### Master Claude skills to build

Deduped across all eight plans. This is the hand-off — these six skills run ~80% of the presence. Each assists a human who reviews and posts; none auto-publishes to social without a human in the loop.

| # | Skill name | Purpose | Platforms served | Priority |
|---|---|---|---|---|
| 1 | **`changelog-to-post`** | The core engine. Takes a git changelog / release notes and drafts platform-native variants (length, register, hashtags/flair, formatting) in one pass: Reddit post, X/Bluesky/Mastodon thread, Discord announce, GitHub Release body, YouTube devlog description, short-form caption. Enforces the voice guide per platform. | Reddit, X, Bluesky/Fediverse, Discord, GitHub, YouTube, FB | **P0** |
| 2 | **`community-reply-triage`** | Pulls unread mentions/comments/issues across channels, classifies (bug / question / praise / hostile / promo-opportunity), drafts honest human-toned replies, flags what a human MUST answer personally, and never auto-sends. Includes the "convert support thread → GitHub issue" flow. | All 8 | **P0** |
| 3 | **`crash-clip-factory`** | Generates the repeatable "which mod crashed you?" short-form assets: picks a format from a template library, writes the hook + on-screen text + shot list from a screen recording, and specifies the edit. The visual-money-shot pipeline. | YouTube, TikTok, Reels, X | **P0** |
| 4 | **`cross-post-repurpose`** | One asset → many. Takes a rendered clip or a written post and produces the platform-correct variants (aspect ratio notes, caption/tag rewrites, crosspost etiquette per sub, Bluesky vs Mastodon length) + a suggested schedule slot. Kills manual reformatting. | TikTok↔Reels↔Shorts↔X, Bluesky↔Mastodon, Reddit crossposts | **P1** |
| 5 | **`github-funnel-keeper`** | Runs the repo as community funnel: drafts Release-as-announcement notes, maintains issue/PR templates, proposes good-first-issue labels + onboarding comments, and keeps Discussions seeded. Semi-autonomous OSS hygiene. | GitHub (feeds Discord + devlog) | **P1** |
| 6 | **`content-calendar-orchestrator`** | The scheduler/brain. Maintains the rolling calendar, tells the dev what's due today across channels, tracks the clip vault + post backlog, and enforces cadence realism (won't over-schedule a solo dev). Ties skills 1–5 together. | All 8 | **P2** |

*Deliberately merged away:* per-platform "changelog→post" skills (all → #1), per-platform reply/triage skills (all → #2), YouTube-SEO + short-form-caption skills (folded into #1 and #3), Discord-bot-stack (that's server infra, not a Cowork skill — build the bot separately; #2 covers the support→issue flow). Provenance/"is this legit" explainer content is a *content template* inside #1 and #3, not its own skill.

### 5. Pre-launch checklist & risks

**Pre-launch checklist:**
- [ ] Discord live: onboarding/verification, support channel, GitHub-linked feeds, invite links minted for each channel.
- [ ] GitHub: Discussions on, issue/PR templates, CONTRIBUTING, good-first-issues, 1.0 Release draft written as an announcement.
- [ ] Landing + `/browse` + `/help` current for 1.0; download links and OG image verified.
- [ ] Hero demo (60–90s) recorded, edited, uploaded unlisted, Discord-reviewed.
- [ ] Clip vault stocked: ≥3 launch clips (crash-name, conflict-catch, provenance) exported in all aspect ratios.
- [ ] Reddit account has genuine helpful history in r/farmingsimulator; subreddit self-promo rules re-read; AMA scheduled with mod-team courtesy heads-up.
- [ ] Voice guide + platform register card finalized and loaded into skills #1–#3.
- [ ] Channel handles claimed + consistent branding (Golden Hour palette) on every banner/avatar/thumbnail.
- [ ] Skills #1–#3 (P0) built and dry-run tested on a past changelog.
- [ ] A written "what if a mod author says we mislabeled their build" provenance-dispute response ready (this *will* come up).

**Risks & guardrails:**
- **Astroturfing / self-promo bans** — the biggest real risk. Reddit and Facebook Groups will punish drop-and-run promotion. Mitigation: participate-first, disclose we're the dev, respect each sub/group's ratio and rules, let value threads earn the mention. Baked into skills #1/#2 as guardrail checks, not left to in-the-moment judgment.
- **Trust at the exe boundary** — "why should I run this near my game files?" Mitigation: lead every skeptical audience with reversible + open-source + no-telemetry + no-account, and point at the public repo. Never dismiss the concern; it's legitimate.
- **Provenance overclaim** — calling a legit mod "Modified" or implying "unsafe" would torch author goodwill and invite a defamation-flavored fight. Mitigation: "provenance, not antivirus" framing everywhere; "Unverified ≠ bad"; have the dispute response ready.
- **Solo-dev burnout / cadence collapse** — eight channels is a trap for one person. Mitigation: only three are truly "nailed"; the rest run on the low-cost changelog→post loop. Cadence is realistic-by-design (X/Bluesky 2–3/wk, one clip/wk sustained), and skill #6 actively *prevents* over-scheduling. Better to post less and reliably than to spike and vanish.
- **AI-content norms** — disclose AI assistance where platforms/communities expect it; a human always reviews and posts. Never mass-DM, never automate follows/likes, never buy engagement.
- **Launch-day fumble** — a bad first patch turnaround reads worse than a bug. Mitigation: hold Wk2-Fri open specifically to ship + announce a fast fix from launch feedback; responsiveness is the trust-builder that converts a beta launch into a community.

---

Files referenced for positioning/voice: `f:\Projects\fs25-modding\Silo\CLAUDE.md` (feature set, moat, trust facts) and `f:\Projects\fs25-modding\Silo\landing\index.html` (hero line "The mod manager FS25 should have shipped with," the "which mod crashed you" hook, Golden Hour palette tokens).

---
_Drafted by a fan-out of platform planners + a strategy lead, grounded in Silo's positioning.
Review before acting; treat cadences as targets, not obligations._
