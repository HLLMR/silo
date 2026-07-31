# Screenshots for the landing page

The landing page (`landing/index.html`, live at silo.hllmr.com, synced daily from this
folder by infra's cron) has a "See it" gallery that loads the four images below. Until
they exist it shows a tasteful "Screenshot coming soon" placeholder — so dropping the
files in here (and letting the daily sync run) lights the gallery up. **Filenames must
match exactly.**

## Capture these four

Drop each as a PNG in this folder:

| File | Show | Notes |
|------|------|-------|
| `library.png` | The Library view with a healthy, populated list | Wide window; a category selected in the rail; a few mods visible with icons. The hero of the set. |
| `triage.png` | Crash & log triage after a run that had errors | The panel that names the culprit mod — the "aha" feature. Use a log with a real named error if possible. |
| `browse.png` | The Browse tab with the detail drawer open | Show the per-source cards (GitHub stars, Nexus, ModHub rating) — the cross-source story. UniversalAutoload is a good example mod. |
| `conflicts.png` | Conflict detection flagging a collision | The conflicts panel showing a duplicate map or filltype collision. |

## How to shoot

- **Aspect ~16:10**, landscape. The gallery renders them 2-up, so ~1600×1000 each is
  plenty; 2× (3200×2000) is better for retina. Keep the four consistent in size.
- Use a **real, populated library** (the dev machine has 700+ mods) so it looks alive —
  not an empty first-run state.
- **Light or dark** is fine, but keep all four the **same theme** for a cohesive strip.
- Trim OS chrome (window shadow is OK); no personal info in view (usernames, paths,
  tokens — the GitHub/Nexus panels should show public counts only, not an account token).
- Save as PNG. Optimize if large, but correctness over bytes.

## After dropping them in

Commit to the repo; infra's daily cron copies `landing/` into the live site, and the
gallery fills in on the next sync. To preview locally, just open `landing/index.html`.
