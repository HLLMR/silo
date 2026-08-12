# Silo → Nexus Mods upload package

Everything staged to publish Silo as a tool/utility on Nexus Mods. This folder is
**gitignored** (installers + assets, local only).

## ⚠️ Read first — clear it with Nexus before uploading

Nexus's rules restrict **internet-connected executables**, and specifically say an
**auto-updater doesn't count as "crucial"** and may get a tool removed. Silo is a
net-connected app with an updater — so this is a real risk. **Don't upload cold.**

→ Open **`email-to-nexus.md`** and send that first. Silo being **open source + code-signed**
makes the case strong, and mod managers (MO2, Fluffy, Unity MM) are already hosted there.
Wait for their green light (they may ask you to ship a build with the updater disabled — that's
a quick config change, ask me).

## What's in here

| File | What it's for |
|---|---|
| `email-to-nexus.md` | The permission email to send **first**. |
| `description.bbcode.txt` | The mod-page description — **BBCode** (paste into Nexus's description box; it is *not* Markdown). |
| `metadata.md` | Every form field: name, summary, category, tags, version, permissions, files. |
| `changelog.bbcode.txt` | The version/changelog block (BBCode). |
| `Silo_0.5.0_x64-setup.exe` | **Main file** to upload — the signed installer. |
| `Silo_0.5.0_x64_en-US.msi` | Optional Windows MSI. |
| `images/main-image.png` | The mod's main/feature image. |
| `images/01-…07-*.png` | Gallery screenshots, in order. |

macOS/Linux builds aren't staged here (Nexus FS25 audience is Windows-first); grab them from
the release if you want to offer them: https://github.com/HLLMR/silo/releases/tag/v0.5.0

## Upload steps (after Nexus says yes)

1. **Add a mod** on the FS25 game page (Tools/Utilities category) — or the site-wide Modding
   Tools section if staff direct you there. See placement note in `metadata.md`.
2. **Name / Summary / Version / Category / Tags** — from `metadata.md`.
3. **Description** — paste `description.bbcode.txt` into the description box (BBCode mode).
4. **Images** — upload `images/main-image.png` as the main image, then the `01–07` gallery.
5. **Files** — upload `Silo_0.5.0_x64-setup.exe` as the **Main** file (version `0.5.0`);
   optionally the `.msi`. Add the changelog from `changelog.bbcode.txt`.
6. **Permissions** — state MIT / open source and be generous; point at the GitHub license.
7. Publish.

## Keeping it current (the real cost of a Nexus presence)

Nexus won't track your GitHub releases — **each new Silo version needs a manual re-upload**
here (new file + changelog), or the Nexus copy goes stale. Silo's own auto-updater still points
at GitHub, so a Nexus-downloaded copy self-updates anyway (which is the exact thing Nexus's
rule frowns on — hence the email). Decide whether the extra reach is worth the upload treadmill;
if not, GitHub + silo.hllmr.com stays the home and you skip Nexus.

## Regenerate this package for a new version

Re-run the staging (bump the version in the URLs), or just ask me to rebuild it for the
current release.
