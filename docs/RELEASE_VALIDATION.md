# Release validation records

Per-release evidence that the **packaged** artifact was validated, per `RC_TESTING.md`. Each
entry records the exact artifact, environment, what was tested, and the outcome — closing the
gap between the RC protocol and visible proof it ran.

---

## v0.2.2 — 2026-08-01

### Artifact identity
| Field | Value |
|---|---|
| Version | 0.2.2 (FileVersion + ProductVersion + registry `DisplayVersion` all agree) |
| Source | GitHub release `v0.2.2` (non-prerelease "Latest"), signed; 9 platform assets + `latest.json` |
| Installed binary SHA-256 | `9aaebbdccb7ee7f34379e999fc34cfeeaf5c382ddb3aab40cf45693cc4f7e7ac` (`%LOCALAPPDATA%\Silo\Silo.exe`, 21,170,688 bytes) |
| Install type | Per-user NSIS → `%LOCALAPPDATA%\Silo` |
| Updater manifest | `releases/latest/download/latest.json` reachable (HTTP 200); version 0.2.2; a signature + correct download URL per platform |

> Note: identity confirmed via the Windows version resource + registry, **not** a naïve binary
> string grep — a `grep 0.2.x` on the exe returns a dependency's `0.2.5`, the exact false
> positive `RC_TESTING.md` warns about.

### Environment
- Windows 11 Pro 26100, standard user, no elevation.
- Real library: **728 mods**, organized layout (`mods/archive/<Category>/`), active set projected.

### Static filesystem-integrity audit — ✅ PASS
The "supreme court" check (`RC_TESTING.md`: every byte present, absent, or changed for an
explainable reason), run against the real 728-mod library after the full 0.1.0 → 0.2.2 cycle:

- **728 / 728** archives present; **0** zero-byte; **0** central-directory failures (all 728);
  full-CRC sample (~20 spread across the set) **0** failures.
- **DB `PRAGMA integrity_check: ok`** (snapshot copy).
- **DB ↔ disk consistent:** `mod_cache` = **728** rows == 728 disk archives (no drift).
- Projection in the flat `mods/` root resolves (the user's dev-mod symlink → target OK).
- No orphaned files, no losses, no unexplained deletions.

**Verdict:** no data-loss path exercised in normal steady state; the reversible-projection
model held across a multi-version upgrade on a real library.

### Migration
- DB carries state accumulated across 0.1.0 → 0.2.2; `integrity_check` clean; scan cache
  consistent with disk. Credential migration (token → keychain, DB scrub + VACUUM) was
  validated during the 0.2.0 RC battery; not re-exercised here.

### Observation to investigate (not a hard stop)
- `organized` table has **0** rows while 728 mods are physically organized under `archive/`.
  Every file is present (no loss), but confirm whether Silo sources "Restore vanilla"
  reversibility from disk structure vs. this table, and whether an empty table degrades any
  restore path. Track as a follow-up.

### Pending — interactive Phase 2 battery (requires an operator at the GUI)
Not yet executed against 0.2.2; requires driving the app + force-kills on a **sacrificial**
cloned library (never the real one for destructive ops):
- Kill mid-operation (Organize / Flatten / Activate / Loadout apply / Update / Install / Bisection / Backup / Config save) → forensic diff after each.
- Manufactured filesystem failures (read-only dest/source, locked files, disk full, existing `.bak`/`.part`, source-deleted-between-preview-and-confirm).
- Bad-download abuse (empty / HTML-as-zip / no `modDesc.xml` / truncated / wrong-valid-mod / hash-mismatch).
- User-ownership conflicts (drop a different zip at a projected name; Silo must refuse to delete the user's file).
- Degraded network (SiloAPI down / malformed JSON / timeouts) → library + diagnostics stay usable.
- Fresh-install vs upgrade matrix.

### Sign-off
- Static integrity audit: **PASS** (automated, 2026-08-01).
- Interactive abuse battery: **PENDING** (operator-driven).
- Overall: suitable for **controlled beta**; run the interactive battery before a broad push.
