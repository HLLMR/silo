# Silo — legal documents (DRAFTS)

Draft legal terms for the Silo public site, owned/operated by **HLLMR Ventures LLC**.
These `.md` files are the editable source; they are rendered to styled pages at
`landing/legal/*.html` and are **live at silo.hllmr.com/legal/ with a prominent "DRAFT — NOT
REVIEWED" banner** on each page (published in good faith by the owner's decision, to signal
good-faith compliance while review is pending).

> ## ⚠️ These are AI-drafted templates, not legal advice — still pending attorney review
> Published as clearly-marked drafts, but **not yet reviewed by a lawyer**. Get them reviewed and
> update both the `.md` source and the rendered `landing/legal/*.html` when finalized.
>
> **Filled in:** state of formation + governing law = **Texas, United States**; contact =
> **legal@hllmr.com**; server-log retention = **30 days**; DMCA notices accepted by email.
>
> **Still needs a human:** a physical mailing address for the DMCA **Designated Agent** and a
> formal **U.S. Copyright Office** agent registration (~$6) to actually secure DMCA safe-harbor
> — the takedown page currently operates by email only and says so.

## Documents

| Document | In one line | Draft |
|---|---|---|
| **Privacy Policy** | The Silo Privacy Policy: the app is local-first with no telemetry, analytics, ads, or accounts; it stores data only on your machine and talks to the public catalog API (which processes IP/logs transiently for rate-limiting behind Cloudflare) and optional third-party services you initiate. | [privacy-policy.md](./privacy-policy.md) |
| **Terms of Service** | Terms of Service for Silo, HLLMR Ventures LLC's free, MIT-licensed FS25 mod manager, covering acceptable use of the website, /browse catalog, and public API; the catalog as a link-back index rather than a file host; as-is warranty disclaimer, liability limits, and indemnity; user responsibility for their own game files and installed mods; and third-party source terms. | [terms-of-service.md](./terms-of-service.md) |
| **Disclaimer & Trademarks** | A focused disclaimer for Silo covering no affiliation with GIANTS Software/Farming Simulator, trademark and catalog-content attributions, the "AS IS" and reversible-but-at-your-own-risk framing for file changes, and the key point that provenance/"Verified" is an integrity and origin signal, not a safety guarantee or antivirus scan. | [disclaimer.md](./disclaimer.md) |
| **Copyright & Takedown** | A DMCA-style notice-and-takedown policy for Silo's aggregated mod catalog, which indexes publicly available metadata and links back to the original source without hosting any mod files. | [copyright-takedown.md](./copyright-takedown.md) |

## Reviewer punch-list

_A non-lawyer consistency/gap check to work through with counsel — placeholders to fill, consistency, gaps, accuracy._

I searched the Silo repo, SiloAPI repo, git history, stashes, the scratchpad, and the landing site exhaustively. **The four draft legal documents do not exist as files anywhere in the workspace** — only the one-line descriptions in the task exist. So I cannot quote actual draft text, enumerate real `[bracketed placeholders]`, or verify literal cross-references/effective dates. What follows is a review keyed to the descriptions plus the real product facts I confirmed in the repo (PROVENANCE.md, /browse, SiloAPI ARCHITECTURE/ENRICHMENT). Anything marked ⚠️ must be re-checked against the actual drafts once they're written/committed.

---

# Silo Legal Docs — Pre-Attorney Review Punch-List

> ⚠️ **Blocker:** The draft files (`privacy-policy`, `terms-of-service`, `disclaimer`, `copyright-takedown`) were not found in the repo or scratchpad. This review is from the task descriptions + confirmed product behavior, not from the draft text. Re-run once the drafts are committed (suggest `Silo/landing/legal/` + linked from footer/sitemap).

## 1. Placeholders a human must fill
These are required by the doc set regardless of draft wording — confirm each is present and filled:

- **Legal entity + form:** "HLLMR Ventures LLC" — state of formation/organization (e.g. `Texas LLC`).
- **Entity mailing address** (needed for ToS notices, DMCA agent, and often required by app stores / EU transparency).
- **Contact email(s):** general/legal contact, privacy contact, and abuse/security. Repo uses `hllmr.com`/`silo.hllmr.com` — pick concrete addresses (e.g. `legal@`, `privacy@`, `dmca@`).
- **DMCA designated agent:** name/role, physical address, email, phone — and whether you'll register with the U.S. Copyright Office DMCA Agent Directory (required for safe-harbor; ~$6). ⚠️ verify the takedown doc names a real agent, not a placeholder.
- **Effective date / Last updated** on all four (see §2).
- **Governing law + venue** (state/county) for ToS — should match entity's state.
- **Counter-notice handling** address/email in the takedown policy.
- **Canonical URLs** for each doc so they can cross-link (footer, /help).

## 2. Consistency (verify across all four)
- **Entity name:** use exactly one form everywhere — "HLLMR Ventures LLC" — and define it once as **"HLLMR," "we," "us"**; don't drift to "Silo" as the legal party (Silo is the product). ⚠️ check the disclaimer/takedown don't say "Silo" where they mean the company.
- **Effective date:** all four should share one date, or each carry its own — pick a convention and apply it uniformly. ⚠️ likely inconsistent/missing in drafts.
- **Defined terms:** "Catalog," "the App," "the Website," "/browse," "the API," "Verified/Provenance," "Third-Party Sources" — define once (ideally in ToS) and reuse the *same* capitalized terms in the other three rather than re-defining.
- **Cross-references:** Privacy Policy ↔ ToS ↔ Disclaimer ↔ Takedown should link to each other by their real URLs, and ToS should incorporate the Privacy Policy and Disclaimer by reference. ⚠️ verify links resolve.
- **Domain/brand:** `silo.hllmr.com` and product name "Silo" spelled consistently.

## 3. Gaps (thin/missing for a free OSS app + metadata aggregator)
- **Cookies/local-storage note (Privacy):** The API/browse page is behind Cloudflare — Cloudflare sets functional cookies (e.g. `__cf_bm`) and the site may use `localStorage`. A "no telemetry/no analytics" policy that is silent on Cloudflare's operational cookies is a gap; add a short storage/cookies section.
- **California (CCPA/CPRA):** minimal — a "we don't sell/share personal information; no targeted ads" statement plus the "right to know/delete" pointer is cheap and closes a common gap.
- **EU/UK (GDPR/UK-GDPR):** even for transient IP/rate-limit logs, name a **lawful basis** (legitimate interest — security/abuse-prevention), **retention** ("transient, deleted within N days"), the **processor** (Cloudflare) + international-transfer basis, and a contact for data requests. IP is personal data under GDPR, so "no personal data at all" would be inaccurate (see §4).
- **Minors / age (ToS + Privacy):** add a minimum-age / "not directed to children under 13/16, we don't knowingly collect from them" clause — standard and expected.
- **Limitation-of-liability cap + carve-outs (ToS/Disclaimer):** confirm there's a **liability cap** (often $0–$100 for a free product), the standard **exclusion of indirect/consequential damages**, and jurisdictional carve-outs ("some jurisdictions don't allow…"). For a free MIT app this is the most important protective clause — make sure it's not just an "AS IS" line.
- **Warranty disclaimer scope:** ensure "AS IS / no warranty" covers not only the App but the **Catalog data, links, and third-party downloads**.
- **Third-party terms (ToS/Disclaimer):** name the sources (GitHub, ModHub/GIANTS) and state users are bound by *their* terms — and that Silo isn't responsible for third-party content/availability.
- **Changes-to-terms / severability / entire-agreement / assignment / no-waiver** boilerplate — confirm present in ToS.
- **Indemnity mutual-scope:** confirm the user-indemnity is tied to *their* misuse/uploaded content, not open-ended.
- **Takedown specifics:** confirm the six §512(c)(3) notice elements, a **counter-notification** path, and a **repeat-infringer** statement. Since Silo hosts **no files**, add the accurate nuance: the remedy is **de-indexing the metadata/link**, and complainants should also contact the actual host (GitHub/ModHub).
- **Export/OFAC + governing-law/dispute** clause in ToS (informal dispute step, optional arbitration — your call with counsel).
- **Accessibility/where-to-find:** all four should be linked from the site footer and `/help`; add to `sitemap.xml` (currently only `/`, `/browse`, `/help`).

## 4. Accuracy risks (overreach / contradiction with real behavior)
Confirmed against repo — watch these:

- **"No data collected at all" overreach (Privacy):** The API is rate-limited **per-IP** (`@fastify/rate-limit`) behind Cloudflare, which processes/transiently logs **IP addresses** — that *is* personal data under GDPR. The policy must say "the **app** sends no telemetry/analytics/accounts; the **API/website** transiently processes IP + standard request logs for rate-limiting and abuse-prevention via Cloudflare." Don't let a blanket "we collect nothing" contradict this. ✅ your description already draws this line — verify the draft body actually preserves it and doesn't lapse into absolute language elsewhere.
- **"Verified" must not read as a safety/AV guarantee:** Repo is explicit — *"this is provenance, NOT antivirus"*; badges are "matches the official build" / "modified — N files differ" / "unverified origin," plus a **client-only** structural red-flag scan (bundled `.dll`/`.exe`). Ensure the Disclaimer/ToS describe Verified strictly as **origin + integrity**, never "safe," "malware-free," or "scanned for viruses." This is the highest-risk line item. ✅ matches your disclaimer description.
- **"Links-back, not host" must be airtight (ToS/Takedown):** /browse copy already says it **"does not host"** and points to the **"original source."** Make sure no clause elsewhere implies Silo distributes, mirrors, or warrants mod files. Note SiloAPI design docs contemplate a *future* Phase-B "mirror + safety-scan" and "authors publish directly to Silo" — **do not** let the current legal docs describe a mirroring/hosting model you don't operate yet (and revisit these docs if/when Phase B ships). ⚠️
- **Local-first / reversible file changes (Disclaimer):** "reversible but at your own risk" is accurate to the hardlink-projection model — just confirm it doesn't promise that *every* operation is reversible or that game saves can't be affected.
- **"No accounts" vs OAuth:** Silo uses GitHub OAuth (device flow) for update/auth against third-party services the **user initiates**. Ensure "no accounts" is scoped to *Silo* accounts and the Privacy Policy discloses that initiating those flows sends the user to, and is governed by, the third party. ⚠️ verify.
- **MIT license vs warranty:** ToS "AS IS" should be consistent with the bundled MIT `LICENSE` (it already disclaims warranty) — fine, just keep them non-contradictory.

---

**Bottom line for the human + lawyer:** the conceptual split is sound and matches how the product actually behaves (local-first app, IP-only transient API logging, link-back catalog, provenance-not-antivirus). The real work before filing is (a) **produce/commit the actual draft text** so placeholders can be verified, (b) fill the **entity/agent/contact/date** placeholders in §1, (c) add the **liability cap, minors, cookies/Cloudflare, and basic CCPA/GDPR** sections in §3, and (d) hold the line on the two accuracy traps in §4 — "we collect nothing" and "Verified = safe."
