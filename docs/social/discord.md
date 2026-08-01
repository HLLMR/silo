<!-- Generated launch plan. Part of Silo's social strategy — see README.md. -->

# Silo on Discord

> A full buildable blueprint for Silo's Discord community hub — channel/role tree, verification onboarding, bot stack, GitHub-linked support flow, and a first-week activation plan.

## Discord server (community hub)

Discord is the **flagship** community surface for Silo — the home base every other channel (Reddit, GitHub, the landing page, ModHub comments) funnels into. It is where a confused player who just got "named the mod that crashed you" turns into a bug reporter, a beta tester, and eventually a regular. This doc is written to be built from directly: exact categories, channels, roles, permissions, bots, and a day-by-day launch week.

### Why Discord fits Silo + who we reach + the voice here

**Why it fits.** Silo's killer feature — "which mod crashed me?" — is a *support conversation* by nature. A player pastes a `log.txt` snippet, someone points at the culprit mod, the loop closes. That is Discord's native shape (real-time, threaded, screenshot-friendly) far better than a forum or subreddit. It also gives us:
- A **live triage funnel** where crash logs become GitHub issues with real reproduction data.
- A **beta tester pool** for the macOS/Linux builds we openly admit are experimental and need testers.
- A **provenance/trust signal**: an open, transparent dev community reinforces "no telemetry, no account, audit the source" better than any landing page.
- A place mod **authors** can claim their identity, which matters for the provenance moat (canonical builds).

**Who we reach here.** Two overlapping crowds: (1) the tech-savvy minority (GitHub/Reddit/Discord-native) who become power users, testers, and contributors; (2) FS25 players who followed a link from a Facebook group or a YouTuber and just want their game to stop crashing. The server must serve both without making the casual player feel they walked into a developer's terminal.

**Voice here.** Honest, helpful, a little witty — genuine gen-Z energy is welcome *in the casual channels* (`#off-topic`, `#mod-showcase`, announcements sign-offs) but **support stays plain and kind**. A panicking player with a corrupted save is not the audience for a meme. Golden rule: match the channel. Never fake hype, never pretend the team is bigger than it is (solo/small dev under HLLMR — say so). Disclose AI-assisted content where it's non-obvious (e.g. auto-drafted release notes get a subtle "drafted with AI, reviewed by a human" footer).

---

### Setup steps (server creation + identity)

1. **Create the server.** Name: **Silo** (server name shows in the sidebar; keep it just "Silo"). Community: enable **Community** mode (Server Settings → Enable Community) — this unlocks Announcement channels, Welcome Screen, Rules Screening, Onboarding, and Server Insights analytics. Required for most of the plan below.
2. **Server icon:** the same "S" mark from the landing page (green gradient `#5c8a3a → #395a25`, white "S", 9px-ish rounded square scaled to Discord's circle). **Banner:** the golden-hour furrows motif from silo.hllmr.com. Keep the brand identical to the site so a visitor knows they're in the right place.
3. **Vanity handle / invite:** apply for the vanity URL `discord.gg/silo` once at Level 3 boost (fallback: request `silo` if available via partner/community program; until then use a **named permanent invite** and mask it behind `silo.hllmr.com/discord` — a redirect you control, so the underlying invite can be rotated without breaking published links). Set the primary invite to **never expire, unlimited uses**, created from `#welcome`.
4. **Roles → bot order:** create roles before inviting bots; bot roles must sit **above** the human roles they manage (verification, auto-roles) or role assignment silently fails.
5. **Safety setup:** Server Settings → Safety Setup → set verification level to **Medium** (verified email + 5 min membership before posting), enable the **DM spam filter** and **"Keep this server safe" AutoMod** presets on day one.
6. **Pinned/anchor content** (built in `#welcome`, `#start-here`, `#announcements`):
   - A one-message **"What is Silo?"** card: one line of what it does, the download link (`silo.hllmr.com`), the source link (`github.com/HLLMR/silo`), and the honesty trust line ("free, open source, no account, no telemetry, reversible by design").
   - A pinned **"How to get support fast"** message in `#get-help`: how to grab your `log.txt`, what to include, and that pasting a log is the fastest path.
   - A pinned **rules** message (mirrored into the Rules Screening gate).

---

### Structure — the full channel + role tree

Ordering matters: read-only orientation up top, support in the middle (highest-traffic, easiest to find), community below, dev/meta at the bottom. Casual players should hit **welcome → get-help** without scrolling.

#### Category & channel tree

```
▸ 🌾 START HERE                         (everyone read-only; onboarding lands here)
   #welcome            — greeting + the "What is Silo?" card + invite. Read-only.
   #rules              — full rules; mirrored into Rules Screening. Read-only.
   #announcements      — ANNOUNCEMENT type. Releases, downtime, big news. Read-only, followable.
   #changelog          — ANNOUNCEMENT type. Auto-posted per GitHub release by the bot. Read-only.
   #roles              — self-assign role menu (reaction/button roles). Read-only except the menu.

▸ 🛠 SUPPORT                            (the funnel — most visible category)
   #get-help           — general "it's not working / how do I…". Forum channel preferred.
   #crash-triage       — paste your log.txt; we name the culprit. Forum channel (one post = one crash).
   #bug-reports        — confirmed bugs; posts here get promoted to GitHub issues. Forum channel.
   #feature-requests   — ideas; up-voted with a reaction; triaged to GitHub Discussions/issues. Forum.
   #faq                — read-only, curated answers (install warnings, SmartScreen, symlink/admin, etc.).

▸ 🚜 COMMUNITY
   #mod-showcase       — show your loadout / a mod Silo saved you from. Media-friendly.
   #general            — on-topic FS25 + Silo chat.
   #off-topic          — anything else; where gen-Z energy is fully allowed.
   #screenshots        — auto-thread on image; pretty farms & Silo screenshots.

▸ 🧪 TESTING & DEV                      (beta + contributor facing)
   #beta               — for @Beta Tester: pre-release builds, test asks. Gated.
   #mac-linux-testers  — the experimental-platform crew we openly recruit. Gated to a role.
   #dev-log            — dev's public build journal / "what I'm working on". Read-only, dev posts.
   #contributors       — for @Contributor + @Mod Author: PRs, provenance/canonical-build coordination. Gated.
   #github-feed        — read-only firehose: commits, PRs, issues, releases via webhook.

▸ 🔒 STAFF (private)
   #mod-team           — moderator coordination.
   #triage-queue       — bot drops flagged/AutoMod items + support items awaiting a human.
   #bot-logs           — audit + bot command output.
```

Notes for the builder:
- **Forum channels** for `#crash-triage`, `#bug-reports`, `#feature-requests`, `#get-help`: each issue becomes its own post with tags (`open`, `needs-log`, `confirmed`, `fixed`, `github-linked`, platform tags `windows`/`mac`/`linux`). This is what makes the support→GitHub flow clean and searchable, and stops the "wall of scrolling messages" problem.
- `#announcements` and `#changelog` are **Announcement channels** so other FS25 servers can *follow* them and mirror Silo releases into their own servers — free distribution.
- `#faq` seeded from the landing page's FAQ + the SmartScreen/Gatekeeper "unsigned app" warning, the symlink/Developer-Mode/admin note, and "does it move my files" (reversible-by-design answer).

#### Role scheme

Colors pulled from Silo's palette so roles read as on-brand.

**Staff / functional (top of list, hoisted):**
| Role | Color | Who | Notes |
|---|---|---|---|
| `@Silo Dev` | gold `#e3b23c` | the maintainer(s) | hoisted, the face of the project |
| `@Moderator` | green `#5c8a3a` | trusted community mods | hoisted |
| `@Bots` | soil `#8a6a45` | all bots | not hoisted; positioned above managed roles |

**Earned / identity (hoisted where it adds signal):**
| Role | Color | How assigned | Notes |
|---|---|---|---|
| `@Contributor` | green-300 `#9cc06e` | manual / GitHub-merge based | merged a PR, wrote docs, triaged |
| `@Mod Author` | barn `#b34a38` | manual + light verification | verified they own a mod in the catalog; matters for provenance/canonical builds |
| `@Beta Tester` | sky `#4a90c2` | self-assign in `#roles`, opt-in | grants `#beta` |
| `@Mac/Linux Tester` | sky, lighter | self-assign in `#roles` | grants `#mac-linux-testers`; we actively recruit these |
| `@Regular` | subtle green | auto after activity (bot) | small perk, recognizes stickiness; optional |

**Onboarding / gate:**
| Role | Color | Notes |
|---|---|---|
| `@Verified` | none (default) | granted at the verification gate; unlocks everything past `#welcome`/`#rules` |
| `@Unverified` | none | default on join; can only see the gate |

**Self-assign notification/interest roles** (no color, in `#roles`): `@Release Pings`, `@Beta Pings`, `@Dev-Log Pings`, and platform tags `@Windows` / `@Mac` / `@Linux` so support can @-ping only the relevant platform when a platform-specific build drops.

**Permission spine:** `@everyone` sees only `#welcome` + `#rules`. The verification gate grants `@Verified`, which opens `START HERE`, `SUPPORT`, `COMMUNITY`. `TESTING & DEV` sub-channels are unlocked per opt-in role. `STAFF` is `@Moderator`/`@Silo Dev` only. Every write-restricted channel is explicitly read-only for `@Verified` (announcements, changelog, dev-log, github-feed, faq).

---

### Onboarding, rules & verification gate

Keep the gate **light** — a crashing player will bounce off a heavy CAPTCHA maze. The goal is bot-spam defense, not friction.

1. **Rules Screening** (native Community feature): a short list a member must accept before they can talk. Rules (concise, human):
   1. Be decent. No harassment, hate, or slurs.
   2. Support is a kindness, not a right — be patient; the dev is one person.
   3. Keep it legal: no piracy links, no cracked mods, no "how do I steal a paid mod."
   4. Respect mod authors' work and licenses. This community backs the people who make mods.
   5. No spam, no unsolicited DMs, no self-promo without asking a mod first.
   6. English in the main channels so we can help (regional channels can come later).
   7. Silo is free and open — no one here will ever ask you to pay or for your password.
2. **Verification gate:** a **single-click button role** (via Carl-bot/MEE6 verification, or a purpose bot) in `#welcome` → grants `@Verified`. Backed by Discord's Medium verification level (verified email + 5-minute wait) so we get bot resistance without a puzzle. Escalate to a reaction-CAPTCHA **only if** a raid happens (AutoMod raid-protection can toggle this).
3. **Onboarding (native Community Onboarding):** the "Customize Your Server" flow — pick interest roles (`@Windows`/`@Mac`/`@Linux`, `@Beta Tester`, ping preferences) and get pointed at `#get-help` and `#crash-triage`. Default channels shown to a new member: `#welcome`, `#announcements`, `#get-help`, `#general`.
4. **Welcome message:** a friendly greeter (bot) drops a short DM or `#welcome` line — warm, not corporate: *"Welcome to Silo. If a mod's been crashing you, head to #crash-triage, paste your log.txt, and we'll name the culprit. If you're just here to hang, #off-topic is that way."* Disclose the greeter is automated.

---

### Moderation approach

- **AutoMod first line:** Discord native AutoMod — block invite links (anti-raid), the default profanity/slur lists (tuned so normal salty gaming talk isn't nuked), mention-spam limits, and **raid protection** (join-gate + activity alerts). Flagged messages route to `#triage-queue`.
- **Bot backstop:** an all-in-one mod bot (Carl-bot **or** Wick) for auto-mute on spam, anti-raid, message logging to `#bot-logs`, and a `warn/mute/ban` ladder with a public reason.
- **Human tone:** small server, so moderation is high-touch and forgiving. Documented **3-strike ladder** (warn → timeout → ban) posted in `#mod-team`; piracy and harassment are instant-remove. Every mod action logged.
- **Piracy stance is load-bearing:** Silo's whole pitch is *legitimacy and provenance*. Zero tolerance for cracked-mod / paid-mod-theft talk protects the brand and mod-author trust. Say this openly in the rules.
- **Conflict of interest / honesty:** mods disclose they're staff; no astroturfing Silo elsewhere from the community; no vote-brigading Reddit/Nexus/ModHub from here (violates those platforms' rules and our own guardrails). If someone asks the community to go upvote something, a mod shuts it down.

---

### Bot stack

Keep it lean — four bots, each with one job, plus native AutoMod. Prefer **native Discord features** over bots wherever they exist (Onboarding, Rules Screening, Announcement-follow) to reduce failure surface.

| Bot | Job | Why this one |
|---|---|---|
| **Native AutoMod + Onboarding + Rules Screening** | Spam/raid defense, interest-role onboarding, rules gate | First-party, no downtime, no data leaves Discord |
| **Carl-bot** (or MEE6) | Reaction/button **role menu** in `#roles`, welcome greeter, auto-`@Regular`, logging, backup mod tools | Best-in-class reaction roles + reliable |
| **GitHub webhook + a small "Silo Herald" bot** | `#github-feed` firehose (commits/PRs/issues) **and** the polished `#changelog`/`#announcements` release posts. The raw webhook feeds `#github-feed`; **Silo Herald** (our own tiny bot, see skills) turns a release into a human, on-brand announcement | We control the release-post voice; raw webhook is too noisy for announcements |
| **Support-triage bot ("Silo Support Bot")** — our own | In forum support channels: on a new `#crash-triage` post, auto-reply with the "paste your full log.txt + Silo version + OS" checklist; detect when a log is attached; let a mod run `/promote-to-github` on a `#bug-reports` post to open a linked GitHub issue and back-link it; sync issue status → forum tags | The core support↔GitHub seam; described in skills below |
| **Ticket option (optional, later):** Ticket Tool / Tickety | Private support tickets for sensitive stuff (a user's log with a machine path they don't want public) | Only if forum channels prove too public |

**Ordering reminder:** the two custom bots and Carl-bot need their role above `@Verified`/`@Beta Tester`/`@Mac-Linux Tester` to assign them.

---

### How support flows connect to GitHub issues

This is the seam that makes Discord *productive* rather than just a chatroom.

1. Player opens a post in `#crash-triage` (forum) → **Silo Support Bot** auto-replies with the checklist and tags the post `needs-log`.
2. Player attaches `log.txt` → bot detects the attachment, swaps the tag to `open`, pings `@Moderator`/helpers.
3. A helper (or the player, if Silo already named the culprit) identifies the mod. If it's a **user/mod problem**, it's answered and the post is tagged `resolved` and archived. If it's a **Silo bug**, it moves to `#bug-reports`.
4. In `#bug-reports`, a mod runs `/promote-to-github` → the bot opens a GitHub issue in `HLLMR/silo` using a template (title, OS, Silo version, log excerpt, repro, link back to the Discord post), applies labels, and posts the issue URL back into the thread. The forum post gets a `github-linked` tag.
5. When the issue **closes** (webhook), the bot flips the forum post to `fixed` and drops a note: *"Fixed in v0.2.3 — grab it at silo.hllmr.com. Thanks for the report."* The release announcement in `#changelog` can @-mention the reporter's platform ping role.
6. **Feature requests** get a reaction-vote; the top ones are periodically summarized (a weekly skill) and opened as GitHub Discussions/issues, closing the loop transparently so people see their ideas land.

No private data leaves Discord without the reporter's post being public or ticketed; the bot strips absolute machine paths from log excerpts before writing to a public GitHub issue.

---

### Content pillars + cadence

**Pillars:**
1. **Ship news** — releases, changelogs, "fixed in this build" (from GitHub, auto-drafted).
2. **Support wins** — "Silo named the mod" stories, FAQ answers turned into pinned knowledge.
3. **Build-in-public / dev-log** — what the dev is working on, roadmap peeks, the provenance moat explained in plain language.
4. **Community spotlight** — showcased loadouts, screenshots, a helpful member of the week.
5. **Tester recruitment** — ongoing, specific asks for macOS/Linux + beta feedback.

**Cadence (realistic for a solo/small dev, automation-assisted):**
- **Per release (event-driven):** polished `#changelog` + `#announcements` post. This is the heartbeat.
- **2–3×/week:** a `#dev-log` note (what's in progress) — short, can be draft-and-queued.
- **Weekly:** a "community roundup" — top feature requests, a showcased screenshot/loadout, a shout to a helpful member; plus a tester ask if a build is pending.
- **Daily-ish, low effort:** the dev/mods just being *present* in `#get-help` and `#off-topic`. Presence > posting for a small server.
- **Monthly:** a roadmap / "state of Silo" post pinned in `#announcements`.

Guardrail: never manufacture activity. An empty channel answered honestly beats fake chatter. Better to have 4 alive channels than 12 dead ones — the tree above can *launch collapsed* (merge `#general`+`#off-topic`, `#get-help`+`#crash-triage`) and split as volume grows.

---

### First-week launch plan (concrete)

**Pre-launch (build day, before any invite is public):**
- Build the full tree, roles, permissions, gate, AutoMod, bots. Seed `#faq` with 6–8 real answers. Seed `#changelog` by back-posting the v0.2.2 release notes so the channel isn't empty. Post the pinned "What is Silo?" and "How to get support fast" cards. Have 2–3 trusted people already inside so a first visitor sees a *lived-in* room, not a ghost town.

**Day 1 — Doors open.**
- Announcement post (its voice): *"Silo has a home now. If FS25 has ever crashed you and refused to say which mod did it — this is where we figure it out together. Paste your log in #crash-triage, we'll name the culprit. Free, open source, no account. Come in."*
- Drop the invite (`silo.hllmr.com/discord`) into the landing page footer, the GitHub README, and pin it on the Silo subreddit / relevant Reddit posts (respecting each sub's self-promo rules).
- Dev does a short **`#dev-log` "hi, I'm the person who made this"** post — honest solo-dev framing builds the trust the brand runs on.

**Day 2 — Support proof.**
- Post a **worked crash-triage example** in `#crash-triage` (a real log → "here's how Silo named this mod") as the template people copy. This teaches the funnel by example.
- FAQ push: answer the top install-friction question publicly (SmartScreen "unsigned app" warning) with the "here's why, here's the source, verify it yourself" honesty.

**Day 3 — Tester recruitment.**
- Pinned ask in `#roles` + `#announcements`: *"macOS and Linux folks — those builds are experimental and we need you. Grab @Mac/Linux Tester, tell us what broke."* Specific, honest about the state.

**Day 4 — Community seed.**
- Kick off `#mod-showcase` with a prompt: *"Show the loadout Silo is currently managing for you — and the one mod it saved you from."* Low-bar, visual, invites replies.

**Day 5 — Build-in-public.**
- `#dev-log`: explain the **provenance moat** in plain language ("how Silo proves a mod is the real build, not tampered") — the differentiator, told as a story, not a spec.

**Day 6–7 — First roundup + listen.**
- First weekly roundup: top feature requests so far, a showcased screenshot, thanks to early helpers. Open the top-voted feature request as a GitHub issue live and link it — proof the loop works.
- Retro: which channels are dead? Collapse them. Which questions repeat? Add to `#faq`.

**Cross-promo (all week, within platform rules):** every other Silo channel points here — Reddit posts, YouTube video descriptions, the landing page, ModHub/Nexus mod pages' comment replies. The Discord is the funnel's floor.

### Claude skills to build

Automations a Claude Code agent ("Cowork") builds so the server runs semi-autonomously. **Every one drafts/queues for a human or performs a bounded, logged action — none mass-DM, spam, or impersonate.** AI-drafted public posts carry a subtle "drafted with AI, human-reviewed" disclosure where norms expect it.

1. **`release-announcer`** — Changelog → Discord post generator.
   - **What it does:** turns a GitHub release into an on-brand `#changelog` + `#announcements` post (Silo voice, grouped "New / Fixed / Known issues", download link, @-mentions the relevant platform ping role). Strips raw commit noise into human copy.
   - **Trigger:** on new GitHub release published (webhook) in `HLLMR/silo`.
   - **Inputs:** release tag, title, body, asset list, linked closed-issue numbers.
   - **Outputs:** a drafted post queued to `#mod-team` for one-click approval → publishes to `#changelog` (Announcement channel, so followers mirror it). Never auto-publishes without approval on major releases; may auto-publish patch notes if flagged low-risk.

2. **`support-triage-assistant`** — the `#crash-triage` / `#bug-reports` first responder.
   - **What it does:** on a new forum support post, replies with the "paste full log.txt + Silo version + OS" checklist, applies `needs-log`; when a log is attached, parses it for the likely culprit mod (same logic family as Silo's `logscan`), suggests a probable cause, and tags `open`; flags posts that look like a real Silo bug vs. a user/mod issue for a human.
   - **Trigger:** on new post / new attachment in the support forum channels.
   - **Inputs:** post content, attached `log.txt`, Silo version, OS.
   - **Outputs:** a threaded reply (clearly bot-authored), tag changes, an escalation ping to `@Moderator` when it detects a probable Silo bug. Never closes a post itself — humans confirm.

3. **`promote-to-github`** — Discord bug → GitHub issue bridge.
   - **What it does:** on a mod's `/promote-to-github` command in `#bug-reports`, opens a labeled issue in `HLLMR/silo` from a template (OS, version, sanitized log excerpt — absolute machine paths stripped, repro, Discord back-link), posts the issue URL into the thread, and tags the post `github-linked`.
   - **Trigger:** slash command by a `@Moderator`/`@Silo Dev` (human-initiated — never automatic).
   - **Inputs:** the forum post, thread messages, invoking mod.
   - **Outputs:** a GitHub issue + a back-link comment in Discord; audit line in `#bot-logs`.

4. **`issue-status-sync`** — GitHub → Discord status closer.
   - **What it does:** when a linked issue changes state (labeled `confirmed`, closed, released), flips the linked forum post's tag and drops a status note ("Fixed in vX — grab it at silo.hllmr.com; thanks @reporter").
   - **Trigger:** on GitHub issue/PR webhook events for issues that carry a Discord back-link.
   - **Inputs:** issue number, new state, releasing version.
   - **Outputs:** forum tag update + a plain status message. Read-only toward GitHub.

5. **`dev-log-drafter`** — build-in-public draft-and-queue.
   - **What it does:** from recent merged PRs / commit messages / a roadmap file, drafts 2–3 short `#dev-log` notes in Silo's voice ("this week I'm working on…"), queued for the dev to edit or discard.
   - **Trigger:** twice weekly (scheduled) + on-demand.
   - **Inputs:** recent git history, roadmap/CLAUDE notes, open milestone.
   - **Outputs:** drafts posted to `#mod-team` for approval. Human ships them.

6. **`weekly-roundup`** — community digest generator.
   - **What it does:** compiles the top up-voted `#feature-requests`, a candidate showcased screenshot/loadout from `#mod-showcase`, notable helpers, and any pending tester ask into one drafted roundup post.
   - **Trigger:** weekly (scheduled).
   - **Inputs:** reaction counts on feature-request posts, `#mod-showcase` activity, resolved support posts.
   - **Outputs:** a drafted roundup queued for approval; on approval posts to `#announcements` and, if a feature crossed a vote threshold, suggests opening a GitHub Discussion/issue for it.

7. **`faq-curator`** — repeat-question → FAQ builder.
   - **What it does:** scans resolved support posts for repeated questions, clusters them, and drafts new `#faq` entries (or flags an existing entry as stale after a release).
   - **Trigger:** weekly (scheduled).
   - **Inputs:** archived/resolved support posts, current `#faq` content, latest release notes.
   - **Outputs:** drafted FAQ additions/edits queued to `#mod-team`.

8. **`showcase-clip-cutter`** — screenshot/clip prep for cross-posting.
   - **What it does:** takes a community-approved `#mod-showcase` or `#screenshots` submission (with the poster's consent recorded), crops/reformats it and drafts a caption for reuse on other Silo channels (Reddit, the landing "community" section). Never reposts without the author's explicit opt-in.
   - **Trigger:** on-demand (a mod reacts a "feature this" emoji on a post).
   - **Inputs:** the image/clip, author handle, consent flag.
   - **Outputs:** a formatted asset + caption draft in `#mod-team`, credited to the author.

9. **`moderation-digest`** — safety summarizer (assist, not enforce).
   - **What it does:** summarizes AutoMod/`#triage-queue` items into a short daily digest so a human mod can act quickly; highlights possible raids or piracy talk.
   - **Trigger:** daily (scheduled) + real-time ping on raid-level events.
   - **Inputs:** AutoMod flags, join-rate anomalies, `#triage-queue` contents.
   - **Outputs:** a digest in `#mod-team`. Takes **no** enforcement action itself — humans/AutoMod enforce.

---

### Metrics that matter

- **Time-to-first-response** in `#crash-triage`/`#get-help` (the promise is "we name your culprit" — speed is the product).
- **Support→resolution rate** and **support→GitHub-issue conversion** (proof the funnel produces real fixes).
- **Verified-member retention** (join → still active at 7/30 days) via Server Insights — vanity metrics like raw member count matter less than *active* members.
- **Beta/tester role opt-ins**, especially `@Mac/Linux Tester` (a stated product need).
- **Announcement-follow count** (other servers mirroring `#changelog`) = free distribution reach.
- **Feature-requests shipped** (community idea → GitHub → release), the clearest "this community shapes the product" signal.

### Guardrails (bake in)

- No astroturfing, fake accounts, vote manipulation, or engagement-buying — including no brigading Reddit/Nexus/ModHub *from* this server.
- Respect Discord ToS + each linked platform's self-promo rules when cross-posting.
- Bots assist and draft; a human approves anything public and anything that touches GitHub. No mass-DMs — the welcome greeter is the only automated DM and it's a single, non-promotional welcome (with an opt-out).
- Be transparent this is the dev/community (solo/small dev — don't fake a big team). Disclose AI-assisted drafts where norms expect it.
- Sanitize user data: strip absolute machine paths from any log excerpt before it leaves Discord for a public GitHub issue; offer tickets for users who don't want a log public.

### Effort / priority

- **P0 (launch-blocking, ~1 build day):** server + Community mode, channel/role tree, verification gate, AutoMod, `#faq` seed, pinned cards, native Onboarding, GitHub webhook → `#github-feed`, `release-announcer` + `support-triage-assistant` skills. These make the funnel work on day one.
- **P1 (launch week):** `promote-to-github` + `issue-status-sync` (the GitHub seam), Carl-bot role menu, `weekly-roundup`, first-week content calendar.
- **P2 (weeks 2–4):** `dev-log-drafter`, `faq-curator`, `moderation-digest`, `showcase-clip-cutter`, ticket bot if forums get too public, vanity URL once boosted.

---
[← Back to the social strategy index](./README.md)
