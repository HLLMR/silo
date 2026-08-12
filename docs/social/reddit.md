<!-- Generated launch plan. Part of Silo's social strategy — see README.md. -->

# Silo on Reddit

> A self-promo-rule-compliant Reddit launch plan for Silo: participate first, lead with the "which mod crashed you?" hook, run a launch post + AMA in r/farmingsimulator and adjacents, and automate changelog-to-post drafting and comment triage with Cowork.

## Reddit

### Why Reddit fits Silo

Reddit is where FS players go **after something breaks**. When the game crashes on load, someone Googles the error and lands on a 2-year-old r/farmingsimulator thread titled "game won't start after adding mods." That is Silo's entire pitch — "which mod crashed you?" — sitting unanswered in search results. We can be the answer.

It's also the one platform where a technical, honest, open-source tool is *rewarded* instead of scrolled past. Redditors distrust polish and love a dev who shows up in the comments, shares source, and says "no telemetry, no account, here's the repo." That's exactly our voice.

**Who we reach here:** the tech-savvy minority of the FS audience — the people who already run 100+ mods, know what a `log.txt` is, post load-order questions, and become the power users who evangelize Silo in the Facebook/Discord groups we *can't* astroturf. Reddit is the top of the credibility funnel, not the volume funnel.

**Voice for Reddit:** Peer, not brand. First-person ("I built this because…"), plain, a little dry-witty, radically honest about limitations (beta, Windows-first, macOS/Linux experimental). Lead with the problem, not the product. Never marketing-speak. Disclose that you're the dev in every promo post. No Gen-Z affectation here — Reddit smells it. Save that energy for TikTok/Discord.

---

### Subreddit map

**Primary**
- **r/farmingsimulator** (~200k+) — the home sub. Everything routes through here. Read the rules pinned in the sidebar *before* posting; most FS subs restrict self-promo to specific days or require flair. Assume a self-promo ratio and a "no reposting your own tool weekly" norm.

**Adjacent / secondary (lower volume, higher-fit for specific posts)**
- **r/farmingsimulator25** — game-specific splinter sub if active; cross-target for FS25-only content.
- **r/opensource** — for the "I built a free, no-telemetry, open-source mod manager" angle. This crowd cares about the provenance/verification moat and reversibility. Great for the launch story.
- **r/selfhosted / r/degoogle-adjacent privacy subs** — only if a post genuinely fits the "no account, no telemetry, everything reversible" angle. Don't force it.
- **r/rust** and **r/sveltejs** — *dev-story* posts only ("built a Tauri v2 + Svelte 5 desktop app that hardlinks 700 mods"), never product promo. These build maintainer credibility and can recruit contributors.
- **r/pcgaming / r/gaming** — do NOT launch here cold. Only relevant much later with a genuinely broad-appeal artifact. Skip for launch.

**Rule of engagement:** Each sub has its own culture and self-promo rule. Treat r/farmingsimulator as the one that matters; treat everything else as opportunistic and only when the post is a *native fit*, not a copy-paste.

---

### The self-promo-compliant way to show up

Reddit's site-wide guidance and most subs enforce a version of the **9:1 rule** — for every 1 post about your own thing, ~9 genuine contributions (comments, answers, non-promo posts) that have nothing to do with promoting Silo. We bake this in as a hard gate, not a suggestion.

**Phase 0 — Earn the account (2–3 weeks BEFORE launch):**
1. Use a real, transparent handle tied to the project identity (see Setup). Fill out the profile so it's obviously "the Silo dev," not a throwaway.
2. Every day, find 1–2 threads where someone is fighting a mod crash, load-order mess, or "which mod broke my save" problem. **Actually help them** — read their log symptoms, name likely culprits, explain bisection *by hand*. Do NOT mention Silo yet, or mention it only when someone explicitly asks "is there a tool for this?" and even then link the repo plainly, once.
3. Bank goodwill and karma. By launch you want a comment history that proves you're a member, not a marketer.

**The ratio in practice:** track it. If the ratio dips below ~9 helpful actions per 1 promo post, pause promo and go help. This is the single most important guardrail on this platform — violating it gets you shadowbanned and burns the sub's goodwill permanently.

**Hard guardrails (Reddit-specific):**
- No alt accounts, no vote manipulation, no asking friends/Discord to upvote a specific post (that's brigading — link the sub, never the post, if you mention it elsewhere).
- Always disclose "I'm the dev" in promo posts and AMAs.
- Respect per-sub self-promo days/flair. Message mods *before* launch to ask if a launch/AMA is welcome — modding-adjacent tools are usually fine, but asking first turns mods into allies.
- AI-assisted content: our posts are human-written and human-posted. If a post uses AI-generated art or copy, say so where it matters. The automations below **draft and queue for a human** — nothing auto-posts to Reddit.

---

### Account & profile setup

**Handle suggestions** (pick one, keep it consistent everywhere):
- `u/silo_app` — clean, obvious.
- `u/hllmr_dev` — ties to the HLLMR identity across GitHub/site; reads as "a person," which Reddit prefers over a brand account.
- `u/silo_dev` — best of both: clearly the maintainer of a named tool.
- Recommendation: **`u/silo_dev`** as the promo/AMA voice, transparently "the dev behind Silo."

**Profile setup:**
- Display name: "Silo — FS25 mod manager (dev)".
- Avatar/banner: the Silo mark in the "Golden Hour" green/gold palette from the landing (`--green-600 #4a7330`, `--gold-500 #e3b23c`).
- Bio: one honest line — "I make Silo, a free & open-source FS25 mod manager. It names the mod that crashed you. No account, no telemetry. github.com/HLLMR/silo".
- Profile pinned posts (Reddit lets you pin up to ~4 to your profile): pin the launch post, the "which mod crashed you" how-to, the AMA, and the repo/download link.
- Turn ON post/comment notifications so you can reply fast on launch day (the algorithm and the community both reward a dev who answers within minutes).

---

### Content pillars

1. **"Which mod crashed you?" (the hook)** — crash triage, log-reading, conflict stories. Silo's signature. Highest-converting, most native to the sub's real pain.
2. **Help-first participation (the ratio fuel)** — answering load-order, install, update, and "will these mods work together" questions with genuinely useful, tool-agnostic advice. Silo mentioned only when asked or clearly relevant.
3. **Build-in-public / dev log** — honest changelog highlights, "here's what shipped in v0.2.x," roadmap, "what should I build next" polls. Feeds r/farmingsimulator lightly and r/rust/r/sveltejs occasionally.
4. **Provenance & trust** — the moat, framed as player-protective: "how do you know a downloaded mod wasn't tampered with?" This differentiates Silo and fits r/opensource + privacy angles.
5. **Respect-the-authors** — spotlight mod authors, clarify Silo never rehosts or cracks CDN gates (ModHub stay index-only), and that everything is reversible. Defuses the #1 objection modders will raise.

---

### Posting cadence (realistic for a solo dev)

- **Daily (Phase 0 and ongoing):** 1–2 genuinely helpful comments in crash/load-order threads. This is non-negotiable ratio maintenance, ~10 min/day.
- **Weekly:** at most **1** Silo-tied post to r/farmingsimulator (a tip, a fix walkthrough, a "shipped this" note) — and only if the 9:1 ratio is healthy. Some weeks the right move is zero promo posts and pure participation.
- **Bi-weekly / per-release:** a changelog-highlight post when a release has something players actually feel (a new triage feature, a fixed crash class) — not every patch.
- **Monthly:** one bigger swing — a dev-story crosspost (r/rust/r/opensource), a poll, or an AMA (quarterly for AMAs, not monthly).

Golden rule: **participation is the floor, promotion is the exception.** When unsure whether to post promo, don't — go answer a question instead.

---

### First-week launch content

**Pre-launch (the week before):** DM the r/farmingsimulator mods: "Hi, I'm the dev of a free open-source FS25 mod manager. I'd like to do a launch post and possibly an AMA — what's your preferred way for tools like this to be shared here?" This one message is worth more than any tactic.

**Day 1 — The launch post (r/farmingsimulator):**
- Title options (test the hook, not the product):
  - "I got tired of guessing which mod crashed my game, so I built a tool that reads the log and names it. Free & open source."
  - "Your game crashed on load. Silo reads log.txt and tells you exactly which mod did it. (Free, no account, no telemetry.)"
- Body: short first-person origin story → the 3 things it does that hurt most (names the crash culprit, catches conflicts *before* launch, one catalog across ModHub/GitHub with the latest version) → honest beta/Windows-first disclosure → "I'm the solo dev, AMA in the comments" → repo + download link. Include a GIF/screenshot of the `◆ diagnose` panel naming a culprit mod. Reply to *every* comment for the first 24–48h.

**Day 2–3 — The signature how-to (works as a standalone value post even if nobody installs):**
- "How to actually find which mod crashed you (with or without my tool)" — teach reading `log.txt`, the disable-half bisection method by hand, common instant-crash causes (duplicate active map). Then: "or Silo does all this in two clicks." This post gives value first, converts second, and is deeply on-brand and rule-safe.

**Day 3–4 — Provenance/trust post (crosspost to r/opensource):**
- "You download a mod. How do you know it wasn't tampered with? I built byte-exact verification against a canonical source hash." Explains Verified / Modified / Unverified. Recruits the privacy/OSS crowd and contributors.

**Day 5–7 — The AMA:**
- Title: "I'm the solo dev of Silo, a free open-source FS25 mod manager (names the mod that crashed you, verifies mods aren't tampered with). AMA." Schedule it, tell the mods, seed 3–4 honest FAQ-style questions in the body (roadmap, why not just use JSG ModHub, macOS/Linux status, how provenance works), then answer live for a few hours. Pin to profile.

**Throughout week 1:** keep the daily help comments going. The launch week is when the ratio matters MOST — a launch post surrounded by a wall of genuine help reads as "a community member shipped something," which is the whole game.

---

### Handling the modding community respectfully

Mod authors are the load-bearing wall of FS. Get this right or the sub turns on you:
- **Never rehost, repackage, or bypass CDN gates.** Say so explicitly. Silo's ModHub entries are index-only, open-the-page. Lead with this in any thread where an author is present.
- **Everything is reversible** — hardlink projection, undoable writes. Say it whenever someone worries Silo "messes with my files."
- **Credit and spotlight authors**, don't compete with them. Silo is plumbing, not a mod site.
- **Invite authors in:** provenance verifies *their* builds against tampering — that protects their reputation. Frame it as a feature *for* authors (users can prove they're running your real build, not a virus-laced reupload).
- If an author objects to their mod appearing in the catalog, respond fast, respectfully, and honor opt-out. Publicly. That single interaction sets the sub's tone toward Silo.

---

### Claude skills to build

Automations a Cowork agent builds so a human can run Reddit semi-autonomously. **Nothing here auto-posts to Reddit** — every skill drafts, queues, and hands to the human for review/post. This respects Reddit ToS (no bot posting), the 9:1 ratio (a human decides *whether* to post), and honesty (a human owns every word).

- **reddit-changelog-to-post**
  - **What:** Reads a new GitHub release's tag + notes, decides whether it's "player-felt" enough to warrant a post (skips pure-plumbing patches), and drafts a Reddit-native post — Reddit-flavored title options (hook-first, not feature-first), plain-Markdown body, honest beta disclaimer, repo/download links. Flags which subreddit(s) fit.
  - **Trigger:** on new GitHub release (release webhook / GH Action).
  - **In:** release tag, notes, changed modules; the current 9:1 ratio state (see ratio-tracker). **Out:** a draft post (title A/B, body, target sub, suggested flair) written to a review queue; a Slack/Discord ping to the human. Never posts.

- **reddit-ratio-tracker**
  - **What:** Maintains the running 9:1 self-promo ledger — logs each helpful comment vs. each promo post, computes current ratio, and gates other skills ("ratio at 4:1 — hold promo, go help"). The compliance backbone.
  - **Trigger:** on any logged Reddit action + weekly rollup. **In:** action log (type: help/promo, permalink, date). **Out:** a ratio dashboard + a boolean "promo-allowed" flag other skills read; weekly summary.

- **reddit-triage-assistant**
  - **What:** Given a pasted thread or a saved-search feed of r/farmingsimulator posts, classifies each as "crash/load-order question we can genuinely help" vs. "promo trap" vs. "author concern," and drafts a *helpful, tool-agnostic* reply (Silo mentioned only when the poster asks for a tool). Prioritizes the day's best 1–2 help opportunities.
  - **Trigger:** daily (morning), or on-demand with pasted content. **In:** subreddit new/rising feed or pasted thread. **Out:** ranked list of threads + draft comments (with a "mentions Silo? y/n and why" note) to the review queue. Human edits + posts.

- **reddit-diagnose-story-cutter**
  - **What:** Turns a real (anonymized, consented) crash-triage session — a log Silo diagnosed — into a "which mod crashed you" case-study post + a captioned screenshot/GIF of the `◆ diagnose` panel. Scrubs usernames/paths.
  - **Trigger:** manual, when a good triage happens; or weekly digest of interesting logs. **In:** sanitized log + Silo diagnosis output. **Out:** draft post + cropped/annotated media asset. Human reviews for privacy before posting.

- **reddit-ama-prep**
  - **What:** Assembles an AMA kit — drafts the title, the seeded FAQ questions (roadmap, why-not-JSG-ModHub, OS status, provenance explainer), and a ready-answers doc pulled from CLAUDE.md/docs so live answers are fast and consistent. During the AMA, drafts reply suggestions for pasted questions.
  - **Trigger:** manual (quarterly AMA). **In:** current roadmap, docs, recent changelog. **Out:** AMA body + answer bank + live draft-reply helper. Human posts and answers.

- **reddit-mention-watcher**
  - **What:** Watches for "Silo," "mod manager," "which mod crashed," and competitor names across target subs; surfaces threads where a helpful (non-promo) reply is warranted or where someone's asking a question Silo answers. Feeds triage-assistant.
  - **Trigger:** hourly/daily poll. **In:** keyword + subreddit list. **Out:** deduped alert list to the queue, tagged (help-fit / author-concern / promo-fit / ignore). Never replies itself.

- **reddit-post-scheduler**
  - **What:** Holds approved drafts and reminds the human of the ideal post window (per-sub self-promo day, peak-activity hour, and only if ratio-tracker says promo-allowed). A reminder + one-click-to-clipboard, not an auto-poster.
  - **Trigger:** on approved draft + time-of-day. **In:** approved queue, per-sub rules, ratio flag. **Out:** "post this now" reminder with the final text; logs the action back to ratio-tracker once the human confirms posted.

---

### Metrics that matter

- **Leading (health, not vanity):** launch-post upvote ratio (>90% is the real signal on Reddit, not raw score), comment count and *sentiment*, and how many comments are questions we can answer (engagement quality).
- **Ratio compliance:** helpful-actions ÷ promo-posts, kept ≥ ~9:1. This is a metric *and* a guardrail.
- **Funnel:** referral traffic from reddit → silo.hllmr.com (UTM `?ref=reddit`), GitHub stars/clones in the 48h after a post, download clicks.
- **Community trust:** mod-author reactions (positive/neutral/negative), unsolicited mentions of Silo by *other* users in later threads (the real win — organic advocacy).
- **AMA:** questions asked, watch for it hitting the sub's top-of-week.

Avoid vanity: raw upvotes and follower count mean little. A 200-upvote post with 80 real questions answered beats a 2,000-upvote post you didn't engage with.

### Guardrails recap

No alts, no vote manipulation, no brigading (link subs, never posts). Disclose "I'm the dev" every time. Honor per-sub self-promo rules and message mods first. Never rehost mods or bypass CDNs; honor author opt-outs publicly and fast. Every automation drafts-and-queues for a human — nothing auto-posts. Be honest about beta status and OS limits in every post.

### Effort & priority

- **P0 / do first (highest ROI):** Phase-0 daily help participation + the "which mod crashed you" how-to post. Pure value, rule-safe, builds the account. Low effort, compounding.
- **P0:** the launch post + live comment engagement (one intense week).
- **P1:** AMA (quarterly), provenance crosspost to r/opensource.
- **P1 automations:** reddit-ratio-tracker and reddit-triage-assistant first (they keep us compliant and consistent), then reddit-changelog-to-post.
- **P2:** diagnose-story-cutter, mention-watcher, ama-prep, post-scheduler.
- **Ongoing / lowest-glamour-highest-value:** the daily help comments. If only one thing gets done, it's this.

---
[← Back to the social strategy index](./README.md)
