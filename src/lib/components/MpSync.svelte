<script lang="ts">
  // Multiplayer mod-set sync: export a manifest of your active set to share, or verify
  // your set against a friend's — turning FS's "mod mismatch" join error into a fix-list.
  import {
    mpExport,
    mpVerify,
    collectionExport,
    collectionImportPreview,
    ghStatus,
    openExternal,
  } from "../api";
  import type {
    MpModRef,
    MpVerifyReport,
    CollectionExportResult,
    ImportPlan,
    GhStatus,
  } from "../types";

  interface Props {
    /** The active set as (techName, path, kind, version) refs. */
    active: MpModRef[];
    /** The whole installed library as (techName, version) — for import bucketing. */
    installed: { techName: string; version: string | null }[];
    onClose: () => void;
  }
  let { active, installed, onClose }: Props = $props();

  let busy = $state<string | null>(null);
  let note = $state<string | null>(null);
  let error = $state<string | null>(null);
  let report = $state<MpVerifyReport | null>(null);

  const zipCount = $derived(active.filter((m) => m.kind === "zip").length);
  const dirCount = $derived(active.length - zipCount);

  // ── Share as a link (Collection) ──
  let canGist = $state(false);
  let collName = $state("");
  let shareBusy = $state(false);
  let shareErr = $state<string | null>(null);
  let shareResult = $state<CollectionExportResult | null>(null);
  let copied = $state(false);

  async function refreshGh() {
    try {
      const s: GhStatus = await ghStatus();
      canGist = s.canGist;
    } catch {
      canGist = false;
    }
  }
  refreshGh();

  async function doShare() {
    if (!collName.trim()) return;
    shareBusy = true;
    shareErr = null;
    shareResult = null;
    copied = false;
    try {
      shareResult = await collectionExport(collName.trim(), null, active);
    } catch (e) {
      shareErr = String(e);
    } finally {
      shareBusy = false;
    }
  }

  async function copyLink() {
    if (!shareResult) return;
    try {
      await navigator.clipboard.writeText(shareResult.url);
      copied = true;
    } catch {
      copied = false;
    }
  }

  // ── Open a shared link (import preview) ──
  let importUrl = $state("");
  let importBusy = $state(false);
  let importErr = $state<string | null>(null);
  let plan = $state<ImportPlan | null>(null);

  async function doPreview() {
    if (!importUrl.trim()) return;
    importBusy = true;
    importErr = null;
    plan = null;
    try {
      plan = await collectionImportPreview(importUrl.trim(), installed);
    } catch (e) {
      importErr = String(e);
    } finally {
      importBusy = false;
    }
  }

  async function doExport() {
    busy = "Hashing & saving…";
    error = null;
    note = null;
    report = null;
    try {
      const n = await mpExport(active);
      if (n != null) note = `Saved a manifest of ${n} mods. Share the .silomp file with your group.`;
    } catch (e) {
      error = String(e);
    } finally {
      busy = null;
    }
  }

  async function doVerify() {
    busy = "Hashing your set & comparing…";
    error = null;
    note = null;
    report = null;
    try {
      report = await mpVerify(active);
    } catch (e) {
      error = String(e);
    } finally {
      busy = null;
    }
  }
</script>

<div class="mp">
  <div class="mp-head">
    <div>
      <h2>Multiplayer mod-set sync</h2>
      <p class="sub">
        Everyone in an FS25 session needs the same mods, same versions, same files —
        that's the #1 "can't join" cause. Match sets here instead of guessing.
      </p>
    </div>
    <button class="x" title="Close" onclick={onClose}>✕</button>
  </div>

  {#if busy}<div class="busy">{busy}</div>{/if}
  {#if error}<div class="err">{error}</div>{/if}
  {#if note}<div class="ok-note">{note}</div>{/if}

  <div class="cards">
    <div class="card">
      <div class="card-title">Host — share your set</div>
      <p class="card-body">
        Exports a manifest of your {active.length} active mods (name, version, content
        hash). Send the file to whoever's joining.
      </p>
      <button class="btn primary" onclick={doExport} disabled={!!busy || active.length === 0}>
        Export active set →
      </button>
    </div>
    <div class="card">
      <div class="card-title">Joiner — check you match</div>
      <p class="card-body">
        Open the host's manifest; Silo compares it to your active set and tells you
        exactly what to add, update, or turn off.
      </p>
      <button class="btn" onclick={doVerify} disabled={!!busy || active.length === 0}>
        Verify against a manifest…
      </button>
    </div>
  </div>

  {#if dirCount > 0}
    <p class="caveat">
      {dirCount} active mod{dirCount === 1 ? " is" : "s are"} unpacked folder{dirCount === 1 ? "" : "s"}
      (dev mods) — those can't be byte-verified and are marked unmatched in a manifest.
    </p>
  {/if}

  <div class="share">
    <div class="card-title">Share as a link</div>
    <p class="card-body">
      Publishes a Collection — the list of your {zipCount} packaged mods, each pinned to a
      version and a content hash — to your own GitHub as a secret gist, and gives you a link.
      Whoever opens it in Silo can install and verify the exact set. No mod files are uploaded,
      only the list.
    </p>

    {#if !canGist}
      <p class="caveat">
        Turn on <b>Enable collection sharing</b> in Settings → GitHub first — it grants the
        <code>gist</code> permission so the link is saved to your account.
      </p>
    {:else if shareResult}
      <div class="ok-note">
        Collection created with {shareResult.count} mod{shareResult.count === 1 ? "" : "s"}.
        {#if shareResult.omitted.length > 0}
          {shareResult.omitted.length} dev-mod folder{shareResult.omitted.length === 1 ? " was" : "s were"}
          left out (no fixed bytes to pin).
        {/if}
      </div>
      <div class="share-link">
        <a class="mn link" href={shareResult.url} onclick={(e) => { e.preventDefault(); openExternal(shareResult!.url); }}>
          {shareResult.url}
        </a>
        <button class="btn" onclick={copyLink}>{copied ? "Copied ✓" : "Copy link"}</button>
      </div>
      <p class="caveat">
        This is a <b>secret</b> gist: unlisted, but <b>not</b> password-protected — anyone with the
        link can see the list. Share it only with your group.
      </p>
    {:else}
      <div class="share-form">
        <input
          class="share-input"
          placeholder="Collection name (e.g. Weekend Co-op Pack)"
          bind:value={collName}
          maxlength="80"
        />
        <button
          class="btn primary"
          onclick={doShare}
          disabled={shareBusy || !collName.trim() || zipCount === 0}
        >
          {shareBusy ? "Publishing…" : "Create share link →"}
        </button>
      </div>
      {#if shareErr}<div class="err">{shareErr}</div>{/if}
    {/if}
  </div>

  <div class="share">
    <div class="card-title">Open a shared link</div>
    <p class="card-body">
      Paste a Silo collection link someone sent you. Silo shows what you already have, what's
      a different version, what it can install for you, and what you'll need to grab yourself —
      before anything is downloaded.
    </p>
    <div class="share-form">
      <input
        class="share-input"
        placeholder="https://gist.github.com/…"
        bind:value={importUrl}
      />
      <button class="btn primary" onclick={doPreview} disabled={importBusy || !importUrl.trim()}>
        {importBusy ? "Reading…" : "Preview →"}
      </button>
    </div>
    {#if importErr}<div class="err" style="margin-top:10px">{importErr}</div>{/if}

    {#if plan}
      {@const p = plan}
      <div class="plan-head">
        <strong>{p.name}</strong>
        {#if p.author}<span class="rt">by {p.author}</span>{/if}
      </div>
      {#if p.description}<p class="caveat" style="margin-top:4px">{p.description}</p>{/if}

      {#snippet bucket(title: string, rows: typeof p.willInstall, drift = false)}
        {#if rows.length > 0}
          <div class="sec">{title} ({rows.length})</div>
          {#each rows as m (m.techName)}
            <div class="row">
              <span class="mn">{m.techName}</span>
              <span class="rt">
                {#if drift}yours {m.installedVersion || "?"} → collection {m.version || "?"}
                {:else}{m.version || ""}{m.source ? ` · ${m.source}` : ""}{/if}
              </span>
            </div>
          {/each}
        {/if}
      {/snippet}

      {@render bucket("Silo can install these", p.willInstall)}
      {@render bucket("Get these yourself — ModHub/Nexus gate downloads", p.openPage)}
      {@render bucket("Different version — update to match", p.versionDrift, true)}
      {@render bucket("Not in the catalog — find these manually", p.unresolved)}
      {@render bucket("Already in your library", p.alreadyPresent)}

      <p class="caveat">
        Installing collections in-app is coming next — for now this shows you exactly what a
        shared set needs.
      </p>
    {/if}
  </div>

  {#if report}
    {@const r = report}
    {#if r.ok}
      <div class="verdict good"><strong>Your set matches.</strong> You're clear to join.</div>
    {:else}
      <div class="verdict bad">
        <strong>Your set doesn't match yet.</strong> Fix the items below, then re-verify.
      </div>

      {#if r.missing.length > 0}
        <div class="sec">Get these ({r.missing.length}) — the host has them, you don't</div>
        {#each r.missing as m (m.techName)}
          <div class="row"><span class="mn">{m.techName}</span><span class="rt">{m.version ?? ""}</span></div>
        {/each}
      {/if}
      {#if r.versionMismatch.length > 0}
        <div class="sec">Wrong version ({r.versionMismatch.length}) — match the host's</div>
        {#each r.versionMismatch as m (m.techName)}
          <div class="row"><span class="mn">{m.techName}</span><span class="rt">yours {m.got || "?"} → need {m.expected || "?"}</span></div>
        {/each}
      {/if}
      {#if r.hashMismatch.length > 0}
        <div class="sec">Same version, different file ({r.hashMismatch.length}) — re-download to be safe</div>
        {#each r.hashMismatch as m (m.techName)}
          <div class="row"><span class="mn">{m.techName}</span><span class="rt">file differs from host</span></div>
        {/each}
      {/if}
      {#if r.extra.length > 0}
        <div class="sec">Turn these off ({r.extra.length}) — the host doesn't have them</div>
        {#each r.extra as name (name)}
          <div class="row"><span class="mn">{name}</span></div>
        {/each}
      {/if}
    {/if}
  {/if}
</div>

<style>
  .mp {
    padding: 16px 18px 22px;
  }
  .mp-head {
    display: flex;
    justify-content: space-between;
    gap: 12px;
    margin-bottom: 12px;
  }
  .mp-head h2 {
    margin: 0;
    font-family: var(--font-display);
    font-size: 1.35rem;
    color: var(--text);
  }
  .sub {
    margin: 4px 0 0;
    color: var(--text-muted);
    font-size: 0.82rem;
    line-height: 1.5;
    max-width: 52ch;
  }
  .x {
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    font-size: 1rem;
    align-self: start;
  }
  .x:hover {
    color: var(--text);
  }
  .busy {
    color: var(--text-muted);
    font-size: 0.85rem;
    margin-bottom: 8px;
  }
  .err {
    background: color-mix(in srgb, var(--danger) 12%, transparent);
    color: var(--danger);
    border: 1px solid color-mix(in srgb, var(--danger) 35%, transparent);
    border-radius: var(--radius-sm);
    padding: 8px 12px;
    margin-bottom: 10px;
    font-size: 0.85rem;
  }
  .ok-note {
    background: color-mix(in srgb, var(--primary) 12%, transparent);
    color: var(--primary);
    border-radius: var(--radius-sm);
    padding: 8px 12px;
    margin-bottom: 10px;
    font-size: 0.85rem;
  }
  .cards {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 12px;
  }
  .card {
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 12px 14px;
    background: var(--surface-raised);
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .card-title {
    font-weight: 600;
    color: var(--text);
  }
  .card-body {
    margin: 0;
    color: var(--text-muted);
    font-size: 0.85rem;
    line-height: 1.5;
    flex: 1;
  }
  .btn {
    padding: 8px 14px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
    color: var(--text);
    font: inherit;
    font-size: 0.85rem;
    cursor: pointer;
  }
  .btn.primary {
    background: var(--primary);
    color: var(--on-primary);
    border-color: transparent;
    font-weight: 600;
  }
  .btn:disabled {
    opacity: 0.55;
    cursor: default;
  }
  .caveat {
    margin: 12px 0 0;
    font-size: 0.78rem;
    color: var(--text-muted);
  }
  .share {
    margin-top: 18px;
    padding: 14px;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--surface-raised);
  }
  .share .card-body {
    margin: 6px 0 10px;
  }
  .share-form {
    display: flex;
    gap: 10px;
  }
  .share-input {
    flex: 1;
    padding: 8px 12px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
    color: var(--text);
    font: inherit;
    font-size: 0.85rem;
  }
  .share-link {
    display: flex;
    align-items: center;
    gap: 10px;
    margin: 4px 0 4px;
  }
  .link {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--info);
    text-decoration: none;
  }
  .link:hover {
    text-decoration: underline;
  }
  .verdict {
    margin-top: 14px;
    padding: 10px 13px;
    border-radius: var(--radius-sm);
    font-size: 0.9rem;
    border: 1px solid var(--border);
  }
  .verdict.good {
    background: color-mix(in srgb, var(--primary) 12%, transparent);
    border-color: color-mix(in srgb, var(--primary) 35%, transparent);
  }
  .verdict.bad {
    background: color-mix(in srgb, var(--danger) 10%, transparent);
    border-color: color-mix(in srgb, var(--danger) 35%, transparent);
  }
  .sec {
    font-size: 0.72rem;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--text-muted);
    margin: 14px 0 5px;
  }
  .row {
    display: flex;
    justify-content: space-between;
    gap: 10px;
    padding: 5px 8px;
    border-radius: var(--radius-sm);
    font-size: 0.85rem;
  }
  .row:nth-child(odd) {
    background: var(--bg);
  }
  .mn {
    color: var(--text);
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-size: 0.8rem;
  }
  .rt {
    color: var(--text-muted);
    font-size: 0.8rem;
    white-space: nowrap;
  }
</style>
