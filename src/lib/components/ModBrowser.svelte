<script lang="ts">
  // The "Browse" tab: discover mods from the canonical SiloAPI catalog, see which
  // you already have, and install new ones straight into the library.
  import { onMount, onDestroy } from "svelte";
  import {
    browseMods,
    siloapiStats,
    siloapiStatus,
    siloapiModDetail,
    siloapiCategories,
    installRemoteMod,
    onInstallProgress,
    openExternal,
  } from "../api";
  import type { BrowseMod, SiloStats, CatalogModDetail, CategoryCount } from "../types";
  import type { UnlistenFn } from "@tauri-apps/api/event";

  interface Props {
    /** Tech names already in the local library, to flag "in library". */
    installed: Set<string>;
    /** Called after a successful install so the parent can rescan. */
    onInstalled: (filename: string) => void;
  }
  let { installed, onInstalled }: Props = $props();

  let query = $state("");
  let category = $state("");
  let categories = $state<CategoryCount[]>([]);
  let results = $state<BrowseMod[]>([]);
  let stats = $state<SiloStats | null>(null);
  let loading = $state(false);
  let error = $state<string | null>(null);
  let installing = $state<string | null>(null);
  let installedNote = $state<string | null>(null);
  let base = $state("");
  // Live download progress per mod id: { done, total } bytes.
  let progress = $state<Record<string, { done: number; total: number | null }>>({});

  // Detail drawer state.
  let detail = $state<CatalogModDetail | null>(null);
  let detailLoading = $state(false);

  let debounce: ReturnType<typeof setTimeout> | null = null;
  let unlisten: UnlistenFn | null = null;

  async function openDetail(m: BrowseMod) {
    detail = null;
    detailLoading = true;
    try {
      detail = await siloapiModDetail(m.id);
    } catch (e) {
      error = String(e);
    } finally {
      detailLoading = false;
    }
  }

  const SOURCE_LABEL: Record<string, string> = {
    github: "GitHub",
    modhub: "ModHub",
    nexus: "Nexus Mods",
    kingmods: "KingMods",
  };

  function pct(id: string): number | null {
    const p = progress[id];
    if (!p || !p.total) return null;
    return Math.min(100, Math.round((p.done / p.total) * 100));
  }

  function fmtMB(bytes: number): string {
    return (bytes / (1024 * 1024)).toFixed(1);
  }

  function hasLocally(m: BrowseMod): boolean {
    return m.techName != null && installed.has(m.techName);
  }

  async function load() {
    loading = true;
    error = null;
    try {
      results = await browseMods({
        query: query.trim() || undefined,
        category: category || undefined,
        limit: 60,
      });
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
    progress = { ...progress, [m.id]: { done: 0, total: null } };
    try {
      const filename = await installRemoteMod(m.id);
      installedNote = `Installed ${filename}`;
      onInstalled(filename);
    } catch (e) {
      error = String(e);
    } finally {
      installing = null;
      const { [m.id]: _drop, ...rest } = progress;
      progress = rest;
    }
  }

  onMount(async () => {
    unlisten = await onInstallProgress((p) => {
      progress = { ...progress, [p.id]: { done: p.done, total: p.total } };
    });
    try {
      base = await siloapiStatus();
      stats = await siloapiStats();
    } catch {
      /* stats/base are best-effort */
    }
    try {
      categories = await siloapiCategories();
    } catch {
      // Older server without /categories — the filter just stays hidden.
    }
    await load();
  });

  onDestroy(() => unlisten?.());
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
    <div class="bh-controls">
      {#if categories.length > 0}
        <select class="cat-select" bind:value={category} onchange={() => load()}>
          <option value="">All categories</option>
          {#each categories as c (c.category)}
            <option value={c.category}>{c.category} ({c.count})</option>
          {/each}
        </select>
      {/if}
      <input
        class="search"
        type="search"
        placeholder="Search the catalog by title…"
        bind:value={query}
        oninput={onSearch}
      />
    </div>
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
    <div class="empty">
      No mods found{query ? ` for “${query}”` : ""}{category ? ` in ${category}` : ""}.
    </div>
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
            {#if installing === m.id}
              {@const p = progress[m.id]}
              <div class="dl">
                <div class="dl-bar">
                  <div
                    class="dl-fill"
                    class:indet={pct(m.id) === null}
                    style={pct(m.id) !== null ? `width:${pct(m.id)}%` : ""}
                  ></div>
                </div>
                <span class="dl-text tnum">
                  {#if p && p.total}
                    {fmtMB(p.done)} / {fmtMB(p.total)} MB
                  {:else if p}
                    {fmtMB(p.done)} MB…
                  {:else}
                    Starting…
                  {/if}
                </span>
              </div>
            {/if}
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
                title="Show details and sources"
                onclick={() => openDetail(m)}
              >
                Details
              </button>
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}

  {#if detailLoading || detail}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div class="drawer-backdrop" onclick={() => (detail = null)}></div>
    <aside class="drawer">
      {#if detailLoading}
        <div class="empty">Loading…</div>
      {:else if detail}
        {@const d = detail}
        <div class="drawer-head">
          <h3>{d.title}</h3>
          <button class="drawer-x" title="Close" onclick={() => (detail = null)}>✕</button>
        </div>
        <div class="drawer-body">
          {#if d.imageUrl}
            <img class="drawer-img" src={d.imageUrl} alt="" />
          {/if}
          <dl class="facts">
            {#if d.author}<dt>Author</dt><dd>{d.author}</dd>{/if}
            {#if d.latestVersion}<dt>Version</dt><dd class="tnum">{d.latestVersion}</dd>{/if}
            {#if d.category}<dt>Category</dt><dd>{d.category}</dd>{/if}
            {#if d.techName}<dt>Tech name</dt><dd class="mono">{d.techName}</dd>{/if}
            {#if d.trustScore != null}<dt>Trust</dt><dd class="tnum">{d.trustScore}/100</dd>{/if}
          </dl>

          {#if d.description}
            <p class="drawer-desc">{d.description}</p>
          {/if}

          <div class="drawer-sec">Available from</div>
          {#if d.sources.length === 0}
            <p class="drawer-none">No sources recorded.</p>
          {:else}
            <ul class="srcs">
              {#each d.sources as s (s.source + s.sourceUrl)}
                <li>
                  <span class="src-name">{SOURCE_LABEL[s.source] ?? s.source}</span>
                  <button class="src-link" onclick={() => openExternal(s.sourceUrl)}>
                    Open page ↗
                  </button>
                  {#if !s.downloadUrl}<span class="src-note">no direct download</span>{/if}
                </li>
              {/each}
            </ul>
          {/if}

          {#if !hasLocally(d)}
            <button
              class="btn primary drawer-install"
              disabled={installing === d.id}
              onclick={() => install(d)}
            >
              {installing === d.id ? "Installing…" : "Install"}
            </button>
          {:else}
            <div class="drawer-owned">Already in your library</div>
          {/if}
        </div>
      {/if}
    </aside>
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
  .bh-controls {
    display: flex;
    align-items: center;
    gap: 8px;
    flex: 1;
    justify-content: flex-end;
    flex-wrap: wrap;
  }
  .cat-select {
    padding: 9px 10px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface-raised);
    color: var(--text);
    font: inherit;
    font-size: 0.85rem;
    max-width: 220px;
    cursor: pointer;
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
  .dl {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-top: 2px;
  }
  .dl-bar {
    flex: 1;
    height: 6px;
    background: var(--bg);
    border-radius: 999px;
    overflow: hidden;
  }
  .dl-fill {
    height: 100%;
    background: var(--primary);
    border-radius: 999px;
    transition: width 0.2s ease;
  }
  .dl-fill.indet {
    width: 35%;
    animation: indet 1.1s ease-in-out infinite;
  }
  @keyframes indet {
    0% {
      margin-left: -35%;
    }
    100% {
      margin-left: 100%;
    }
  }
  .dl-text {
    font-size: 0.72rem;
    color: var(--text-muted);
    white-space: nowrap;
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

  /* ── Detail drawer ── */
  .drawer-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.35);
    z-index: 40;
  }
  .drawer {
    position: fixed;
    top: 0;
    right: 0;
    bottom: 0;
    width: min(420px, 92vw);
    background: var(--surface);
    border-left: 1px solid var(--border);
    box-shadow: var(--shadow-2);
    z-index: 41;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
  .drawer-head {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 14px 16px;
    border-bottom: 1px solid var(--border);
  }
  .drawer-head h3 {
    margin: 0;
    font-family: var(--font-display);
    font-size: 1.15rem;
    color: var(--text);
    flex: 1;
    line-height: 1.25;
  }
  .drawer-x {
    border: none;
    background: transparent;
    color: var(--text-muted);
    font-size: 1rem;
    cursor: pointer;
    padding: 4px 6px;
    border-radius: var(--radius-sm);
  }
  .drawer-x:hover {
    background: var(--bg);
    color: var(--text);
  }
  .drawer-body {
    padding: 14px 16px 24px;
    overflow-y: auto;
  }
  .drawer-img {
    width: 100%;
    border-radius: var(--radius-sm);
    border: 1px solid var(--border);
    margin-bottom: 12px;
    display: block;
  }
  .facts {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: 4px 12px;
    margin: 0 0 12px;
    font-size: 0.85rem;
  }
  .facts dt {
    color: var(--text-muted);
  }
  .facts dd {
    margin: 0;
    color: var(--text);
  }
  .mono {
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-size: 0.8rem;
  }
  .drawer-desc {
    color: var(--text);
    font-size: 0.88rem;
    line-height: 1.5;
    margin: 0 0 14px;
  }
  .drawer-sec {
    font-size: 0.72rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
    margin-bottom: 6px;
  }
  .drawer-none {
    color: var(--text-muted);
    font-size: 0.85rem;
    margin: 0 0 14px;
  }
  .srcs {
    list-style: none;
    padding: 0;
    margin: 0 0 16px;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .srcs li {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 10px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface-raised);
  }
  .src-name {
    font-weight: 600;
    font-size: 0.85rem;
    color: var(--text);
  }
  .src-link {
    margin-left: auto;
    border: none;
    background: transparent;
    color: var(--info);
    font: inherit;
    font-size: 0.8rem;
    cursor: pointer;
    padding: 0;
  }
  .src-link:hover {
    text-decoration: underline;
  }
  .src-note {
    font-size: 0.72rem;
    color: var(--text-muted);
  }
  .drawer-install {
    width: 100%;
    flex: none;
  }
  .drawer-owned {
    text-align: center;
    color: var(--text-muted);
    font-size: 0.85rem;
    padding: 8px;
  }
</style>
