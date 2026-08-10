<script lang="ts">
  // Nexus source card — link-back only. Counts come from the catalog (the silo-api index
  // Nexus approved); Silo makes NO live Nexus API call and never handles a personal key.
  // Downloads always happen on Nexus's own site, gates respected — this just opens the page.
  import { openExternal } from "../api";

  let {
    version = null,
    sourceUrl,
    downloads = null,
    endorsements = null,
  }: {
    version?: string | null;
    sourceUrl: string;
    downloads?: number | null;
    endorsements?: number | null;
  } = $props();

  function fmt(n: number): string {
    if (n < 1000) return `${n}`;
    return `${(n / 1000).toFixed(n < 10_000 ? 1 : 0)}k`;
  }
</script>

<div class="src-card nexus">
  <div class="src-head">
    <span class="src-name">Nexus Mods</span>
    {#if version}<span class="src-ver tnum">v{version.replace(/^v/i, "")}</span>{/if}
  </div>

  {#if downloads != null || endorsements != null}
    <div class="src-nums tnum">
      {#if endorsements != null}<span title="Endorsements">👍 {fmt(endorsements)}</span>{/if}
      {#if downloads != null}<span title="Downloads">⬇ {fmt(downloads)}</span>{/if}
    </div>
  {/if}

  <div class="src-actions">
    <button class="act" onclick={() => openExternal(sourceUrl)} title="Downloads happen on Nexus">
      Open on Nexus ↗
    </button>
  </div>
</div>

<style>
  .src-card {
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--surface, var(--bg));
    padding: 10px 12px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .src-head {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 10px;
  }
  .src-name {
    font-weight: 700;
    font-size: 13px;
  }
  .src-ver {
    font-size: 12px;
    color: var(--text-muted);
  }
  .src-nums {
    display: flex;
    gap: 14px;
    font-size: 13px;
    color: var(--text);
    flex-wrap: wrap;
  }
  .src-actions {
    display: flex;
    gap: 8px;
  }
  .act {
    flex: 0 0 auto;
    border: 1px solid var(--border);
    background: var(--bg);
    color: var(--text);
    padding: 6px 12px;
    border-radius: var(--radius-sm);
    font-size: 12.5px;
    font-weight: 600;
    cursor: pointer;
  }
  .act:hover {
    border-color: color-mix(in srgb, var(--primary) 45%, var(--border));
    color: var(--primary);
  }
</style>
