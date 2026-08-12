# Email Nexus BEFORE uploading

**Why:** Nexus's [File Submission Guidelines](https://help.nexusmods.com/article/28-file-submission-guidelines)
prohibit internet-connected executables *unless the connection is crucial to the tool's
function*, and they explicitly say **"auto update" does not qualify** and that they may
remove tools that phone home. Silo is a net-connected app **with an auto-updater**, so
uploading it cold risks getting it moderated/removed. Nexus tells you to contact staff
first with your reasoning + source — so do that. Your case is strong (open source + signed).

**To:** support@nexusmods.com (or the "Contact us" form, category: file submission / mod authors)

---

**Subject:** Permission to publish an open-source FS25 mod manager (Silo) that uses the network

Hi Nexus team,

I'd like to publish **Silo**, an open-source (MIT) mod manager for Farming Simulator 25, as
a tool/utility on Nexus, and I want to clear it with you first because it connects to the
internet and I know that's governed by your File Submission Guidelines.

- **Source:** https://github.com/HLLMR/silo (public, MIT)
- **Site:** https://silo.hllmr.com  ·  **Verify/trust page:** https://silo.hllmr.com/trust/
- The Windows installer is Authenticode **code-signed** (verified publisher: David Hellmer).

**Why the network use is crucial to the tool's function:** Silo's core job is to index the
FS25 mod catalog, check installed mods for updates across their sources, and verify that an
installed mod matches the build its author actually published (content-hash provenance). None
of that works offline — the network access *is* the feature, not incidental telemetry.

**On auto-update specifically:** the app has a built-in updater that checks our GitHub
releases. I understand that isn't considered "crucial." I'm happy to (a) disable the updater
in a Nexus-distributed build, or (b) leave update delivery to Nexus's own file versioning —
whichever you prefer. Please let me know.

There are already mod managers hosted under Modding Tools (Mod Organizer 2, Fluffy, Unity Mod
Manager, Amethyst), so I believe Silo fits — I just want to follow the rules rather than
upload and hope. Happy to answer anything or walk you through the source.

Thanks,
David Hellmer (HLLMR)

---

**Likely outcomes**
- ✅ *Approved as-is* → upload with `metadata.md`.
- ✅ *Approved, disable updater* → build a Nexus variant with the Tauri updater turned off (I can do this — it's a config change), then upload.
- ⚠️ *They'd rather it stay off Nexus* → keep GitHub as the home; consider a **link-only presence** isn't really a thing on Nexus, so in that case skip Nexus and lean on the site + ModHub audience instead.
