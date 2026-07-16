<script lang="ts">
  // The "Browse" tab: discover mods from the canonical SiloAPI catalog, see which
  // you already have, and install new ones straight into the library.
  import { onMount } from "svelte";
  import {
    browseMods,
    siloapiStats,
    siloapiStatus,
    installRemoteMod,
    openExternal,
  } from "../api";
  import type { BrowseMod, SiloStats } from "../types";

  interface Props {
    /** Tech names already in the local library, to flag "in library". */
    installed: Set<string>;
    /** Called after a successful install so the parent can rescan. */
    onInstalled: (filename: string) => void;
  }
  let { installed, onInstalled }: Props = $props();

  let query = $state("");
  let results = $state<BrowseMod[]>([]);
  let stats = $state<SiloStats | null>(null);
  let loading = $state(false);
  let error = $state<string | null>(null);
  let installing = $state<string | null>(null);
  let installedNote = $state<string | null>(null);
  let base = $state("");

  let debounce: ReturnType<typeof setTimeout> | null = null;

  function hasLocally(m: BrowseMod): boolean {
    return m.techName != null && installed.has(m.techName);
  }

  async function load() {
    loading = true;
    error = null;
    try {
      results = await browseMods({ query: query.trim() || undefined, limit: 60 });
    } catch (e) {
      error = String(e);
      results = [];
    } finally {
      loading = false;
    }
  }

  function onSearch() {
    if (debounce) clearTimeout(debounce);
    debounce = setTimeout(load, 300);
  }

  async function install(m: BrowseMod) {
    installing = m.id;
    error = null;
    installedNote = null;
    try {
      const filename = await installRemoteMod(m.id);
      installedNote = `Installed ${filename}`;
      onInstalled(filename);
    } catch (e) {
      error = String(e);
    } finally {
      installing = null;
    }
  }

  onMount(async () => {
    try {
      base = await siloapiStatus();
      stats = await siloapiStats();
    } catch {
      /* stats/base are best-effort */
    }
    await load();
  });
</script>

<div class="browse">
  <div class="browse-head">
    <div class="bh-title">
      <h2>Browse mods</h2>
      {#if stats}
        <span class="catalog-count">
          {stats.mods.toLocaleString()} mods · {stats.sources.toLocaleString()} sources
        </span>
      {/if}
    </div>
    <input
      class="search"
      type="search"
      placeholder="Search the catalog by title…"
      bind:value={query}
      oninput={onSearch}
    />
  </div>

  {#if base}
    <p class="source-note">Catalog: {base}</p>
  {/if}

  {#if error}
    <div class="error">{error}</div>
  {/if}
  {#if installedNote}
    <div class="ok-note">{installedNote} — rescanning library…</div>
  {/if}

  {#if loading && results.length === 0}
    <div class="empty">Loading catalog…</div>
  {:else if results.length === 0}
    <div class="empty">No mods found{query ? ` for “${query}”` : ""}.</div>
  {:else}
    <div class="grid">
      {#each results as m (m.id)}
        {@const here = hasLocally(m)}
        <div class="card" class:owned={here}>
          <div class="thumb">
            {#if m.imageUrl}
              <img src={m.imageUrl} alt="" loading="lazy" />
            {:else}
              <div class="thumb-fallback">{(m.title || "?").slice(0, 1)}</div>
            {/if}
            {#if here}<span class="owned-badge">In library</span>{/if}
          </div>
          <div class="card-body">
            <div class="card-title" title={m.title}>{m.title}</div>
            <div class="card-meta">
              {#if m.author}<span class="author">{m.author}</span>{/if}
              {#if m.latestVersion}<span class="ver">v{m.latestVersion}</span>{/if}
            </div>
            {#if m.category}<div class="chip">{m.category}</div>{/if}
            <div class="card-actions">
              {#if here}
                <button class="btn ghost" disabled>Installed</button>
              {:else}
                <button
                  class="btn primary"
                  disabled={installing === m.id}
                  onclick={() => install(m)}
                >
                  {installing === m.id ? "Installing…" : "Install"}
                </button>
              {/if}
              <button
                class="btn ghost"
                title="Open this mod's catalog record"
                onclick={() => openExternal(`${base}/mods/${m.id}`)}
              >
                Details
              </button>
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .browse {
    padding: 16px 20px 40px;
    max-width: 1200px;
    margin: 0 auto;
  }
  .browse-head {
    display: flex;
    align-items: flex-end;
    justify-content: space-between;
    gap: 16px;
    flex-wrap: wrap;
    margin-bottom: 4px;
  }
  .bh-title {
    display: flex;
    align-items: baseline;
    gap: 12px;
  }
  .bh-title h2 {
    font-family: var(--font-display);
    margin: 0;
    font-size: 1.5rem;
    color: var(--text);
  }
  .catalog-count {
    color: var(--text-muted);
    font-size: 0.85rem;
  }
  .search {
    flex: 1;
    min-width: 220px;
    max-width: 420px;
    padding: 9px 12px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface-raised);
    color: var(--text);
    font: inherit;
  }
  .source-note {
    color: var(--text-muted);
    font-size: 0.75rem;
    margin: 2px 0 12px;
  }
  .error {
    background: color-mix(in srgb, var(--danger) 12%, transparent);
    color: var(--danger);
    border: 1px solid color-mix(in srgb, var(--danger) 30%, transparent);
    padding: 8px 12px;
    border-radius: var(--radius-sm);
    margin-bottom: 12px;
  }
  .ok-note {
    color: var(--primary);
    padding: 8px 12px;
    margin-bottom: 12px;
  }
  .empty {
    color: var(--text-muted);
    text-align: center;
    padding: 60px 0;
  }
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    gap: 16px;
    margin-top: 12px;
  }
  .card {
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    overflow: hidden;
    display: flex;
    flex-direction: column;
    box-shadow: var(--shadow-1);
    transition: box-shadow 0.15s, transform 0.15s;
  }
  .card:hover {
    box-shadow: var(--shadow-2);
    transform: translateY(-2px);
  }
  .card.owned {
    opacity: 0.82;
  }
  .thumb {
    position: relative;
    aspect-ratio: 16 / 9;
    background: var(--bg);
    overflow: hidden;
  }
  .thumb img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
  }
  .thumb-fallback {
    width: 100%;
    height: 100%;
    display: grid;
    place-items: center;
    font-family: var(--font-display);
    font-size: 2.4rem;
    color: var(--green-300);
    background: linear-gradient(135deg, var(--green-700), var(--green-900));
  }
  .owned-badge {
    position: absolute;
    top: 8px;
    right: 8px;
    background: var(--primary);
    color: var(--on-primary);
    font-size: 0.7rem;
    font-weight: 600;
    padding: 2px 8px;
    border-radius: 999px;
  }
  .card-body {
    padding: 10px 12px 12px;
    display: flex;
    flex-direction: column;
    gap: 6px;
    flex: 1;
  }
  .card-title {
    font-weight: 600;
    color: var(--text);
    line-height: 1.25;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
  .card-meta {
    display: flex;
    gap: 8px;
    align-items: center;
    font-size: 0.8rem;
    color: var(--text-muted);
  }
  .card-meta .ver {
    margin-left: auto;
    font-variant-numeric: tabular-nums;
  }
  .chip {
    align-self: flex-start;
    font-size: 0.72rem;
    color: var(--soil-700);
    background: color-mix(in srgb, var(--soil-500) 16%, transparent);
    padding: 2px 8px;
    border-radius: 999px;
  }
  .card-actions {
    display: flex;
    gap: 8px;
    margin-top: auto;
    padding-top: 6px;
  }
  .btn {
    flex: 1;
    padding: 7px 10px;
    border-radius: var(--radius-sm);
    border: 1px solid var(--border);
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
  .btn.primary:hover:not(:disabled) {
    background: var(--primary-hover);
  }
  .btn.ghost {
    flex: 0 0 auto;
  }
  .btn:disabled {
    opacity: 0.55;
    cursor: default;
  }
</style>
