<script lang="ts">
  // Nexus source card. Counts (👍 endorsements · ⬇ downloads) read keyless from the
  // v2 GraphQL; the action (👍 Endorse) runs through the user's OWN Nexus personal API
  // key and lands on Nexus's servers. Silo stores nothing beyond the key it was given.
  import { nexusModStats, nexusEndorse, openExternal } from "../api";
  import type { NexusModStats } from "../types";

  let {
    modId,
    version = null,
    sourceUrl,
    connected = false,
    onConnect,
  }: {
    modId: number;
    version?: string | null;
    sourceUrl: string;
    connected?: boolean;
    onConnect?: () => void;
  } = $props();

  let stats = $state<NexusModStats | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let busy = $state(false);

  async function load() {
    loading = true;
    error = null;
    try {
      stats = await nexusModStats(modId);
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }
  $effect(() => {
    modId;
    load();
  });

  function fmt(n: number): string {
    if (n < 1000) return `${n}`;
    return `${(n / 1000).toFixed(n < 10_000 ? 1 : 0)}k`;
  }

  async function toggleEndorse() {
    if (!stats) return;
    if (!connected) return onConnect?.();
    busy = true;
    error = null;
    const next = !stats.youEndorsed;
    try {
      await nexusEndorse(modId, next, version);
      stats.youEndorsed = next;
      stats.endorsements += next ? 1 : -1;
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }
</script>

<div class="src-card nexus">
  <div class="src-head">
    <span class="src-name">Nexus Mods</span>
    <button class="src-link" onclick={() => openExternal(sourceUrl)} title="Open on Nexus">
      {stats?.name ?? `mod ${modId}`} ↗
    </button>
  </div>

  {#if loading}
    <div class="src-body muted">Loading…</div>
  {:else if error && !stats}
    <div class="src-body">
      <span class="err">{error}</span>
      <button class="mini" onclick={load}>Retry</button>
    </div>
  {:else if stats}
    <div class="src-nums tnum">
      <span title="Endorsements">👍 {fmt(stats.endorsements)}</span>
      <span title="Downloads">⬇ {fmt(stats.downloads)}</span>
    </div>

    <div class="src-actions">
      {#if connected}
        <button class="act" class:on={stats.youEndorsed} disabled={busy} onclick={toggleEndorse}>
          {stats.youEndorsed ? "👍 Endorsed" : "👍 Endorse"}
        </button>
      {:else}
        <button class="act connect" onclick={() => onConnect?.()}>
          Add Nexus key to Endorse
        </button>
      {/if}
    </div>
    {#if error}<div class="src-err">{error}</div>{/if}
  {/if}
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
  .src-link {
    border: none;
    background: transparent;
    color: var(--info);
    font-size: 12px;
    font-weight: 600;
    padding: 0;
    cursor: pointer;
    max-width: 60%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .src-link:hover {
    text-decoration: underline;
  }
  .src-body {
    font-size: 12.5px;
    display: flex;
    align-items: center;
    gap: 10px;
  }
  .muted {
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
  .act:hover:not(:disabled) {
    border-color: color-mix(in srgb, var(--primary) 45%, var(--border));
    color: var(--primary);
  }
  .act.on {
    background: color-mix(in srgb, var(--accent, var(--primary)) 16%, transparent);
    border-color: var(--accent, var(--primary));
    color: var(--accent, var(--primary));
  }
  .act.connect {
    color: var(--info);
    border-color: color-mix(in srgb, var(--info) 40%, var(--border));
  }
  .act:disabled {
    opacity: 0.6;
    cursor: default;
  }
  .mini {
    border: 1px solid var(--border);
    background: var(--bg);
    color: var(--text);
    border-radius: var(--radius-sm);
    padding: 2px 8px;
    font-size: 12px;
  }
  .err,
  .src-err {
    color: var(--danger);
    font-size: 12px;
  }
</style>
