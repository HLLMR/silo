<!-- Generated launch plan. Part of Silo's social strategy — see README.md. -->

# Silo on GitHub community

> Turn the HLLMR/silo repo into a community funnel — Discussions, templates, good-first-issues, Releases-as-announcements, and a CONTRIBUTING on-ramp — with Claude skills to run it semi-autonomously.

## GitHub as a social/community surface

### Why GitHub fits Silo (and who we reach here)

For most launch channels, GitHub is where you point people *after* you've earned their curiosity. For Silo it's different: **"open source, no telemetry, audit the code yourself" is the pitch**, and GitHub is where that pitch is either true or it isn't. The landing page, the FAQ, and every release note already send people here to verify the claim. So the repo isn't a code dump — it's the **proof surface and the trust moat** made browsable.

Three things make GitHub uniquely load-bearing for this product:

- **The provenance story lives on code.** Silo's differentiator is verifying a mod is the real, untampered build. A skeptic's first move is "prove *you're* not tampered." An open repo with clean CI, real tests, a SECURITY.md, and human-reviewed releases *is* that proof. No other channel can carry it.
- **The audience splits, and GitHub catches the high-leverage half.** Silo's users are mostly casual/older FS25 players who live in Facebook and Discord — they will never open a repo. But the **mod authors, tinkerers, Linux/Proton players, and the tech-savvy Reddit/GitHub minority** are exactly who file the best bug reports, test the experimental platforms, contribute code, and become the credible voices who vouch for Silo *back in* those Facebook/Discord spaces. GitHub is where you convert a power user into an evangelist.
- **Mod authors are a second, distinct audience with skin in the game.** Silo indexes and *hashes* their work. Some will be curious, some wary ("are you scraping my mod?"). The repo is where you show them the catalog is index+deep-link (respecting their download pages), how provenance protects *them* from reupload/tampering, and how to correct their catalog entry. That relationship is won or lost in Issues and Discussions.

**Who we reach here:** power users and bug-hunters; mod authors; Linux/macOS testers; potential contributors; and journalists/other-project-devs doing due diligence. **Who we don't:** the casual majority — don't design the repo for them, design it as the referable, trustworthy backend that the *other* channels can always point to.

### Voice for this platform

GitHub voice is **the honest-engineer register**: precise, calm, receipts-first. The wit from the landing page ("which mod crashed me?") is welcome in a release headline or a Show-and-tell post, but Issues and PR replies stay warm-and-straight. Concretely:

- **Lead with the artifact, not adjectives.** "Here's the log line that names it" beats "powerful crash detection."
- **Disclose the AI-assisted engineering plainly** (the README already does — keep that tone: architecture/testing/releases are human-owned, source is open to audit). This is a norm-expected disclosure on this platform; never bury it.
- **Never oversell the beta.** macOS/Linux are experimental — say so every time, cheerfully. Under-promising on a trust product *is* the marketing.
- **Treat every bug reporter like a co-author.** They handed you a free QA session. Thank, reproduce, label, and close the loop publicly.
- **Solo-dev honesty.** It's fine to be one person ("HLLMR"). Don't fake a team ("we" is okay as project-voice; never invent staff).

---

### Account / space setup

The org/handle (`HLLMR`) and repo (`HLLMR/silo`) already exist and are healthy. Setup here means finishing the **community-surface** configuration that isn't code.

1. **Org profile polish (`HLLMR/.github` → `profile/README.md`).** Create the special `HLLMR/.github` repo with a `profile/README.md` so the org landing page isn't blank. One short paragraph: who HLLMR is (solo/small dev), what ships under it (Silo, and the FS25 game-mods), links to silo.hllmr.com and Discord. This is the first thing a due-diligence visitor sees.
2. **Repo "About" sidebar.** Set the description to a one-liner ("The FS25 mod manager that names the mod that crashed you. Free, open source, no telemetry."), add the website `https://silo.hllmr.com`, and topics: `farming-simulator`, `fs25`, `mod-manager`, `tauri`, `rust`, `svelte`, `modding`, `gaming`. Topics are a discovery surface — people browse `farming-simulator` on GitHub.
3. **Social preview image (Settings → Social preview).** Upload a 1280×640 card reusing the landing `og.png` art (Golden Hour palette, "names the mod that crashed you"). This is the image every shared repo link renders as in Discord/Twitter/Slack — right now a shared link looks like generic code. High ROI, 20 minutes.
4. **Enable Discussions** and seed the category tree (below). Turn on "Announcements" category (maintainer-post-only).
5. **Enable Sponsors or a FUNDING.yml** (optional, honest): a `.github/FUNDING.yml` pointing to Ko-fi/GitHub Sponsors if HLLMR wants it — framed as "keeps the catalog server on," never as gating features. Silo is free forever; funding is for the VPS.
6. **Pinned content:**
   - **Pin 3 issues** max: the current "Known beta limitations" tracking issue, the "macOS/Linux tester call" issue, and a "Roadmap / what's next" issue.
   - **Pin 3 Discussions:** the Welcome/START-HERE post, the latest release announcement, and the "Show us your loadout / setup" thread.
7. **Labels overhaul** (see structure). Add the community-funnel labels that don't exist yet: `good first issue`, `help wanted`, `platform: linux`, `platform: macos`, `area: catalog`, `area: provenance`, `false-positive`, `mod-author`, `needs-log`, `needs-repro`.
8. **Community Standards checklist** (Insights → Community Standards). Drive it to 100%: it already has README, CoC, Contributing, License, Issue templates, PR template, Security. The remaining gap is usually **the issue-template `config.yml`** (contact links) and **Discussions** — both covered here.
9. **Release settings.** Confirm the release workflow opens a **draft prerelease** (it does), and that the "Set as latest release" logic is correct so the README `github/v/release` badge shows the real newest beta.

---

### Structure (repo as community space)

**A. Discussions category tree** (the "social" half of the funnel):

| Category | Format | Purpose |
|---|---|---|
| 📣 **Announcements** | Announcement (maintainer only) | Release posts, roadmap shifts, platform milestones. Mirrors Releases in a comment-friendly space. |
| 🙏 **Q&A** | Q&A (accepted answers) | "How do I…", install help, "is this mod safe", Gatekeeper/SmartScreen questions. Deflects support out of Issues. Accepted-answer builds a searchable FAQ. |
| 💡 **Ideas** | Open-ended (upvote) | Feature requests that aren't yet actionable issues. Upvotes = a public priority signal. Graduate hot ones into issues. |
| 🚜 **Show and tell** | Open-ended | Loadouts, big libraries tamed, "Silo found my crash," setup screenshots. This is the *community-glue* category — celebrate users. |
| 🧪 **Platform testing** | Q&A | macOS/Linux/Proton reports, corralled away from the main bug tracker until reproducible. |
| 🧩 **Mod authors** | Open-ended | For creators: catalog corrections, provenance questions, "why is my mod flagged Modified," opt-out/opt-in conversations. |

**B. Issue intake structure** (`.github/ISSUE_TEMPLATE/`) — extend the existing `bug_report` + `feature_request` with:

- **`config.yml`** (the missing piece): `blank_issues_enabled: false`, plus contact links routing non-bugs away — "Ask a question → Q&A Discussion," "Share your setup → Show and tell," "Security issue → SECURITY.md," "Chat with us → Discord." This alone dramatically raises issue quality.
- **`crash_triage.yml`** (issue *form*, not markdown): structured fields — OS, Silo version, mod count, the `log.txt` culprit line Silo named, whether bisection was run. Forms produce far more repro-able reports than freeform. Auto-labels `bug`, `needs-log`.
- **`platform_report.yml`**: for macOS/Linux — distro/OS, install method, what discovery found/missed. Auto-labels `platform: linux`/`platform: macos`.
- **`catalog_or_provenance.yml`**: "a mod is missing / wrong version / flagged Modified when it shouldn't be." Auto-labels `area: catalog` or `false-positive`, `mod-author`. Critical: provenance false-positives are reputational risk — make them a one-click report.

**C. Contributor on-ramp** (already strong; add the funnel front-door):

- A curated set of **8–12 `good first issue`s** that are genuinely small and well-specified (a clearer error string, a new conflict-type test fixture, a Linux path-discovery case, a copy-edit in a panel). Each with a "why this matters / where to look / how to test" body — the CONTRIBUTING already documents the check commands, so link them.
- A **`help wanted`** set one tier up (macOS discovery, Proton path detection, a new catalog source) for people who want a meatier task.
- A **pinned "Roadmap" issue** or a lightweight GitHub **Project board** (Backlog / Next / In progress / Shipped) so contributors see where they can plug in. Public roadmap = free credibility and free contributor routing.

**D. README as social hub** (mostly done): the badges are already there. Add a small **"Community" section** near the bottom linking Discussions, Discord, and "good first issues" — so a curious reader has an obvious next click beyond "Download."

---

### Content pillars + cadence

**Pillars** (everything posted here maps to one):

1. **Ship & tell** — every release is an announcement, not a silent tag. The changelog is the raw material.
2. **Proof & transparency** — "here's how provenance works," "here's the source review we acted on," CI/test milestones. Feeds the trust moat.
3. **Community spotlight** — resurface great Show-and-tell posts, thank contributors, "bug of the week that made Silo better."
4. **On-ramp** — periodic "good first issues are open," "we need Linux testers," roadmap updates.
5. **Field notes** — short technical Discussions on FS25 modding gotchas Silo learned (duplicate-map instant-crash, fillType last-wins) — genuinely useful, SEO-friendly, positions Silo as the domain expert. Cross-references the code.

**Cadence** (realistic for a solo dev — this is a *low-volume, high-signal* channel; do not force daily posting):

- **Per release** (the anchor): Release notes + mirrored Announcement Discussion. Whenever a version ships.
- **Weekly:** triage pass — label/answer new issues & Q&A within a few days; convert the best Idea into an issue; accept-answer resolved Q&As. (This is *maintenance*, the real work of the channel.)
- **Bi-weekly:** one substantive post — either a Field-note Discussion, a contributor/Show-and-tell spotlight, or a roadmap nudge.
- **Monthly:** refresh the `good first issue` shelf (close stale, add new); post a short "state of Silo" (downloads, mods indexed, platforms).

### First-week launch content (concrete)

The repo is public and at v0.2.2 already — "launch" here = turning it from a code host into a *staffed community space*. First-week checklist, in order:

1. **Day 1 — "START HERE / Welcome" Announcement Discussion.** Pinned. What Silo is in 3 sentences, the honesty pledge (open, no telemetry, reversible, beta), where to file bugs vs. ask questions vs. show off, and the "we especially need macOS/Linux testers" ask. Hook: *"FS25 gives you a flat folder and a shrug. This is where we fix that in the open."*
2. **Day 1 — Ship the social preview image + repo About/topics.** So every link shared this week renders as a real card.
3. **Day 2 — Release the current beta *as an announcement*.** Re-cut the latest changelog into a proper Release with the beta/unsigned/known-limits framing (the v0.1.0 notes are a perfect template), and mirror it to Announcements. Hook: *"v0.2.2 — Silo now names the mod that crashed you, and tells you if that mod is even the real build."*
4. **Day 2 — Pin the "Known beta limitations" issue** and the **"Call for macOS & Linux testers" issue** (with the exact discovery gaps listed, so a tester knows what to poke).
5. **Day 3 — Seed Show-and-tell** with the maintainer's own post: "Silo taming 728 mods on this machine" + the library screenshot. Gives the category a non-empty first thread so real users feel invited.
6. **Day 3 — Publish 8 `good first issue`s** with full "where to look / how to test" bodies. Post one Q&A/Announcement pointing new contributors at them.
7. **Day 4 — First Field-note Discussion:** "Why two active maps is an instant crash in FS25 (and how Silo catches it before launch)." Links to the `conflicts` module. This is the shareable-to-Reddit artifact.
8. **Day 5 — Mod-authors welcome post** in the Mod authors category: how the catalog indexes (index + deep-link, we don't rehost), how provenance protects *your* release from tampered reuploads, and how to report a wrong entry. Defuses the "are you scraping me" worry before it becomes a hostile issue.
9. **Throughout — respond to everything within 24–48h.** Week one's real deliverable is *responsiveness*: every issue labeled, every question answered, every Show-and-tell reacted to. A community space is judged by whether the first ten visitors got a reply.

---

### Claude skills to build

Automations a Claude Code agent ("Cowork") builds so a solo dev can run this channel semi-autonomously. **Every one drafts/queues for a human to approve — none post, comment, or DM autonomously.** All respect GitHub ToS and the no-astroturf guardrails.

#### `silo-release-announcer`
- **What it does:** Turns a new release into publish-ready announcement copy across surfaces. Reads the `CHANGELOG.md` `[x.y.z]` section + the git tag, and drafts (a) polished GitHub Release notes in the established v0.1.0 house style (highlights → install/first-run → known beta limits → links), (b) a mirrored Announcements Discussion post, and (c) a short cross-post blurb for Discord/Reddit/Facebook the *other* platform skills can pick up. Preserves the beta/unsigned/experimental-platform framing every time.
- **Trigger:** on new GitHub release published (or tag `v*` pushed → draft).
- **Inputs:** `CHANGELOG.md`, git tag, release workflow artifact list (for the assets/checksums line), prior release notes as style reference.
- **Outputs:** draft Release body (markdown), draft Discussion post, cross-post blurb — all written to a review queue / draft PR; nothing published without human approval.

#### `silo-changelog-guard`
- **What it does:** On every PR, checks whether a user-facing change added a `## [Unreleased]` line in the right group (Added/Changed/Fixed/Security), and if missing, drafts a suggested changelog line written *for a player, not a compiler* (per CONTRIBUTING). Posts it as a PR review suggestion for the author to accept.
- **Trigger:** on pull_request opened/synchronized.
- **Inputs:** PR diff, `CHANGELOG.md`, CONTRIBUTING changelog rules.
- **Outputs:** a suggested-change comment (or a skip if the PR is internal-only). Assists; never blocks merge on its own.

#### `silo-issue-triage`
- **What it does:** Reads each new issue and drafts a triage action: suggest labels (`bug`/`platform: linux`/`area: provenance`/`needs-log`/`needs-repro`/`false-positive`/`mod-author`), detect duplicates against open issues, and — if a crash report is missing its `log.txt` culprit line — draft a friendly "could you attach the log line Silo named?" reply. For questions filed as issues, drafts a "this fits better in Q&A Discussions" redirect.
- **Trigger:** on issue opened (+ a weekly sweep over untriaged issues).
- **Inputs:** issue body/title, current label set, open-issue corpus, the issue-form field values.
- **Outputs:** proposed labels + a drafted first-response comment in the review queue for one-click send. No auto-close.

#### `silo-provenance-fp-watch`
- **What it does:** Watches for `false-positive` / provenance-flag reports (the reputationally sensitive ones) and fast-tracks them: pulls the reported mod + version, cross-checks against the SiloAPI canonical hash record, and drafts a maintainer summary ("likely a real re-pack" vs. "canonical hash stale, needs re-ingest") plus a holding reply to the reporter. Flags anything that looks like a genuine mis-flag as high priority.
- **Trigger:** on issue labeled `false-positive` or `area: provenance`.
- **Inputs:** issue details, SiloAPI hash record for the mod, the reporter's provenance output.
- **Outputs:** a private maintainer brief + a drafted reporter acknowledgement. Never auto-resolves a provenance verdict.

#### `silo-good-first-issue-curator`
- **What it does:** Monthly, proposes candidate `good first issue`s by scanning the codebase for small, well-bounded gaps (TODOs, missing test fixtures, a copy string, a platform edge case) and drafts full issue bodies in the "why / where to look / how to test" format, with the CONTRIBUTING check commands linked. Also flags stale open good-first-issues to close.
- **Trigger:** weekly/monthly cron; or on-demand.
- **Inputs:** repo source, existing `good first issue` set, CONTRIBUTING.
- **Outputs:** a batch of drafted issues for the maintainer to approve/label. Keeps the on-ramp shelf stocked.

#### `silo-community-digest`
- **What it does:** Weekly, compiles a maintainer digest: new issues/PRs needing a reply, unanswered Q&A (past 48h), Ideas gaining upvotes worth graduating to issues, and standout Show-and-tell posts worth spotlighting/pinning. Drafts the "reply within 48h" shortlist so nothing goes cold.
- **Trigger:** weekly cron.
- **Inputs:** GitHub Issues/Discussions API (open items, timestamps, reactions/upvotes).
- **Outputs:** a prioritized digest (markdown) + draft replies/spotlight posts for approval.

#### `silo-field-note-drafter`
- **What it does:** Drafts the bi-weekly Field-note Discussion (pillar 5) from a chosen topic, grounding it in the actual code — e.g. reads the `conflicts` module and writes "why duplicate active maps instant-crash FS25, and how Silo catches it," with a link back to the exact source. Produces the genuinely-useful, cross-postable artifact.
- **Trigger:** manual / bi-weekly cron with a topic queue.
- **Inputs:** topic prompt, the relevant Rust module(s), docs/.
- **Outputs:** a drafted Discussion post + a shorter Reddit/Discord cross-post version. Human edits and posts.

#### `silo-social-card-cutter`
- **What it does:** Generates the supporting images this channel needs — the repo social-preview card, per-release header images, and cropped/annotated screenshots (culprit-named triage, conflict list, Verified badge) for release posts and Show-and-tell — reusing the Golden Hour design tokens so everything looks like the product.
- **Trigger:** on release-announcer run, or on-demand.
- **Inputs:** app screenshots, landing-page design tokens/palette, headline text.
- **Outputs:** PNG assets (social card, release header, annotated shots) staged for the maintainer to attach.

---

### Metrics that matter here

Vanity stars are the *least* useful number. Track the funnel, not the applause:

- **Response latency** — median time-to-first-maintainer-reply on issues/Q&A. Target < 48h. This is the single metric that makes or breaks a community space.
- **Issue quality rate** — % of bug reports that arrive reproducible (log line attached, form filled). Rising = the templates/skills are working.
- **Contributor conversion** — unique non-maintainer PR authors; `good first issue` → merged-PR rate; first-time contributors per month.
- **Discussion health** — Q&A with accepted answers; Show-and-tell posts by *real users* (not seeded); Ideas upvotes → issues graduated.
- **Release reach** — release-asset download counts per version (the README badge tracks total); does an announcement move downloads.
- **Provenance trust signal** — false-positive reports resolved and how fast (a trust product must be visibly responsive here).
- **Referral echo** — mentions/links back from Reddit/Discord/Facebook that originated at a repo artifact (a Field-note, a release). GitHub's job is to *feed* the other channels; measure that it does.

### Guardrails

- **No vote/star manipulation, no fake accounts, no seeded "users."** The one maintainer Show-and-tell seed post is disclosed as the maintainer's own — that's demonstration, not astroturf.
- **Every skill assists a human; nothing auto-posts, auto-comments, or DMs.** Drafts land in a review queue. This is both a guardrail and a quality bar.
- **Disclose AI-assisted content where norms expect it** — the README's existing disclosure is the standard; carry it into any substantive generated post.
- **Respect mod authors' surfaces** — catalog is index + deep-link, never rehost; give authors a first-class correction/opt-out path; treat provenance flags as claims to verify, never accusations.
- **Never overstate platform support** — macOS/Linux experimental, stated every time.
- **Security issues go through SECURITY.md, not public issues** — the issue-template `config.yml` must route them there.
- **Honor GitHub ToS and rate limits** in every automation; back off, don't hammer the API.

### Rough effort / priority

| Item | Effort | Priority |
|---|---|---|
| Social preview image + About/topics | 30 min | **P0** — every shared link this week depends on it |
| Enable Discussions + seed category tree + Welcome post | Half day | **P0** — this *is* the community surface |
| Issue-template `config.yml` (routing) | 30 min | **P0** — deflects noise, hits 100% Community Standards |
| Release the current beta as a proper announcement | 2 hrs | **P0** — the launch's anchor content |
| 8 curated `good first issue`s + pin tester/limits issues | Half day | **P1** — contributor + tester funnel |
| Crash/platform/catalog issue *forms* | Half day | **P1** — raises report quality, feeds triage skill |
| `HLLMR/.github` org profile README | 1 hr | **P1** — due-diligence first impression |
| `silo-release-announcer` + `silo-social-card-cutter` skills | 1 day | **P1** — the highest-leverage automations |
| `silo-issue-triage` + `silo-community-digest` skills | 1 day | **P2** — keeps response latency low as volume grows |
| `silo-changelog-guard`, `silo-provenance-fp-watch` skills | 1 day | **P2** — quality + trust safety nets |
| `silo-good-first-issue-curator`, `silo-field-note-drafter` | 1 day | **P3** — steady-state content/on-ramp engine |

---
[← Back to the social strategy index](./README.md)
