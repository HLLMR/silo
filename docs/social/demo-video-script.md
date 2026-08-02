# Silo — full-length demo video walkthrough

A follow-along beat sheet for a **narrated screen-share** on your own machine (your real
~728-mod library is the best demo material there is). Target length **~10–12 minutes**.

**Golden rules**
- Talk like you're showing a friend, not reading ad copy. The `SAY` bullets are *points*, not a
  script — say them in your own words. Ramble a little; it reads as real.
- **Show, then say.** Do the thing on screen first, then explain what just happened.
- Lead with the crash-diagnosis. It's the whole pitch — everything after is "and it also…".
- Be honest about the beta/AI stuff near the end. Owning it is your strongest move.

---

## Pre-flight (do this before recording)
- OBS/Loom at **1080p, 30–60 fps**; capture the Silo window (or full screen if you'll alt-tab to FS25).
- **Maximize Silo.** Close the **Settings/account** panels so no personal handle is on screen.
- Mic check; kill notifications (Focus Assist / Do Not Disturb).
- Have a **real crash to show.** Best: a `log.txt` from a session that actually crashed (Silo reads
  it live). If you don't have one handy, the crash-triage panel on your current log still works —
  narrate whatever it finds. Don't fake a crash; if it's "Healthy," say so and lean on bisection.
- Optional 5–10s **talking-head cold open** recorded separately ("Hey, I'm ___, I built this because…").

---

## Beat sheet

### 1 · Hook — the pain (0:00–0:30)
- **DO:** Open on the FS25 crash / the wall-of-red `log.txt`, or the huge `mods/` folder. If you did a
  talking-head cold open, splice it here.
- **SAY:**
  - "If you run Farming Sim with a pile of mods, you know this moment — the game dies on load and
    won't tell you which of your [N] mods did it."
  - "The usual fix is disabling mods half at a time and praying. I got sick of it and built Silo."

### 2 · The money shot — crash diagnosis (0:30–2:30)  ← the star
- **DO:** Show the Silo **Library** with your real mod count front and center. Click **◆ diagnose**.
  Let it parse the log and name the culprit. Open the offender, **park** it. Show the count drop /
  the "healthy" state. If you can, relaunch and show it load.
- **SAY:**
  - "This is Silo. That's my actual library — [N] mods." (let the number land)
  - "When the game crashed, instead of guessing, I hit diagnose. It reads the log, throws out the
    cosmetic noise, and points at the mod most likely at fault."
  - "One click to park it, relaunch, and we're in."
- **If the log can't name it → guided bisection:**
  - **DO:** Open **guided bisection**; show it disabling half, prompting a relaunch, narrowing.
  - **SAY:** "When the log isn't clear, Silo automates the disable-half-and-relaunch hunt — and it's
    crash-safe, it snapshots your active set so you never lose your setup."

### 3 · How it keeps [N] mods sane (2:30–4:00)
- **DO:** Toggle a mod **active/parked** (the dot). Show the **Organize** preview if you have loose
  mods. Point at the archived library vs the flat game folder.
- **SAY:**
  - "Everything you own lives here, filed by category. The game only ever sees the set you mark
    active — Silo projects it in with links, so there's no copying and nothing's duplicated."
  - "And it's reversible by design: one button puts everything back to a plain vanilla mods folder."

### 4 · Catch the crash *before* it happens — conflicts (4:00–5:00)
- **DO:** Open **conflicts**. Show a real one from your set (duplicate map, or a filltype/script
  collision). Explain the severity.
- **SAY:**
  - "Two active maps is an instant crash on load — Silo catches that before you ever launch, plus
    filltypes, vehicle types, and scripts that quietly collide across your set."

### 5 · Is this mod the *real* build? — verify / trust (5:00–6:30)  ← the moat
- **DO:** Open a mod's detail drawer; run the **integrity check**. Show a **✓ Verified** result (and,
  if you have one, a **Modified** with the exact changed files).
- **SAY:**
  - "Here's the part I haven't seen anywhere else. Silo hashes your installed mod and checks it
    against the real build its source published. Verified means byte-for-byte the same. Modified
    tells you exactly which files differ."
  - "It's provenance, not antivirus — it proves *what* a mod is, it doesn't guess intent. And it
    works across ModHub, GitHub, and Nexus."

### 6 · One catalog, every source (6:30–7:30)
- **DO:** Open **Browse**. Search a mod. Show it pulling ModHub/GitHub/Nexus into one record with the
  latest version. If it's a GitHub mod, show the in-app install with the progress bar.
- **SAY:**
  - "A mod might live on ModHub, get newer builds on GitHub, and a mirror on Nexus. Silo pulls them
    into one record so you actually see the latest version across all three — not whichever site
    updated last."

### 7 · A setup per playthrough — loadouts (7:30–8:15)
- **DO:** Show **Loadouts** — save the current set, switch to another, show the active set change.
- **SAY:** "I keep different mod sets for different saves — a realism run, a chill run. One click to
  swap the whole set."

### 8 · "…and it also does" — rapid fire (8:15–9:45)
Keep each to ~15–20s. Don't dwell — this is the "there's a lot here" montage.
- **Savegames:** "Edit a save's settings without launching the game."
- **Multiplayer sync:** "Host exports a hashed mod-set; your friends verify theirs matches before
  they join — no more 'you have a different version' roulette."
- **Filltype bridge:** "The sugar-beet fix — generates a tiny companion mod so your equipment accepts
  a map's stubborn filltype, no vehicle edits."
- **Bindings map:** "Your whole control map in one place, with inputs bound to more than one action
  flagged."

### 9 · Why you can trust it (9:45–10:45)
- **DO:** Nothing fancy — just talk. Maybe show the `/trust` page or the "no telemetry" line.
- **SAY:**
  - "It's free and open source — every line of the app is on GitHub. No account, no telemetry;
    catalog searches and update checks send only what they need, nothing about your library."
  - "And it's careful with your files. It never deletes something it can't prove it created — if you
    swapped your own build in somewhere, Silo leaves it alone."
  - **(own the AI part):** "Yeah, I built this with a lot of AI help. The architecture, the security
    calls, the testing, the releases — those are mine, human-reviewed. The source is right there;
    don't take my word for it, read it."

### 10 · Outro / CTA (10:45–11:15)
- **DO:** Show `silo.hllmr.com`. Mention the Discord.
- **SAY:**
  - "It's a beta, Windows-first, Mac and Linux experimental. Grab it at silo.hllmr.com, come break it
    in the Discord, and tell me what mod folder finally made it fall over."

---

## Delivery tips
- **One take, keep the flubs.** A small stumble is more legit than a polished read. Edit out only the
  dead air (Descript/CapCut auto-trim silences — that's fine AI help).
- **Let numbers breathe.** Say "[N] mods," then pause and let the viewer see it.
- **Cursor discipline.** Move deliberately; don't wiggle. Click, wait a beat, then talk.
- **Captions.** Auto-generate them (accessibility + silent autoplay on socials). Proofread names.
- **Cut a 30–45s vertical clip** of Beat 2 (the crash diagnosis) for Shorts/Reels/TikTok — that's the
  hook that travels. (See the platform plans in this folder.)

## What NOT to do
- No synthetic avatar, no AI voiceover, no AI b-roll on the hero content — for an AI-assisted product,
  real screen + your real voice is the thing that legitimizes it. Keep AI to invisible production
  help (script, captions, trimming, thumbnail).
