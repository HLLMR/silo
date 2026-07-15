<script lang="ts">
  import { onMount } from "svelte";
  import { defaultModsPaths, scanMods, onScanProgress } from "./lib/api";
  import type { ModEntry, ScanResult } from "./lib/types";
  import VirtualList from "./lib/components/VirtualList.svelte";
  import ModRow from "./lib/components/ModRow.svelte";
  import CategoryRail from "./lib/components/CategoryRail.svelte";

  let roots = $state<string[]>([]);
  let mods = $state<ModEntry[]>([]);
  let scanning = $state(false);
  let progress = $state({ done: 0, total: 0 });
  let result = $state<ScanResult | null>(null);
  let query = $state("");
  let errorMsg = $state<string | null>(null);
  let selected = $state<{ category: string | null; subcategory: string | null }>({
    category: null,
    subcategory: null,
  });

  const q = $derived(query.trim().toLowerCase());
  const filtered = $derived.by(() => {
    let list = mods;
    if (selected.category) {
      list = list.filter(
        (m) =>
          m.category === selected.category &&
          (!selected.subcategory || m.subcategory === selected.subcategory),
      );
    }
    if (q !== "") {
      list = list.filter(
        (m) =>
          (m.title ?? "").toLowerCase().includes(q) ||
          m.techName.toLowerCase().includes(q) ||
          (m.author ?? "").toLowerCase().includes(q),
      );
    }
    return list;
  });

  const stats = $derived.by(() => {
    let maps = 0,
      scripts = 0,
      unique = 0,
      issues = 0;
    for (const m of mods) {
      if (m.isMap) maps++;
      if (m.scriptCount > 0) scripts++;
      if (m.uniqueType) unique++;
      if (m.error || m.ignoredDigitPrefix) issues++;
    }
    return { maps, scripts, unique, issues };
  });

  const pct = $derived(
    progress.total > 0 ? Math.round((progress.done / progress.total) * 100) : 0,
  );

  async function runScan() {
    if (scanning) return;
    scanning = true;
    errorMsg = null;
    progress = { done: 0, total: 0 };
    try {
      const r = await scanMods(roots.length ? roots : undefined);
      result = r;
      mods = r.mods;
    } catch (e) {
      errorMsg = String(e);
    } finally {
      scanning = false;
    }
  }

  onMount(() => {
    let unlisten: (() => void) | undefined;
    (async () => {
      unlisten = await onScanProgress((p) => (progress = p));
      try {
        roots = await defaultModsPaths();
      } catch (e) {
        errorMsg = String(e);
      }
      if (roots.length) runScan();
    })();
    return () => unlisten?.();
  });
</script>

<div class="app">
  <header class="topbar">
    <div class="brand">
      <div class="logo">S</div>
      <div>
        <h1>Silo</h1>
        <p class="tagline">Farming Simulator 25 mod library</p>
      </div>
    </div>

    <div class="path" title={roots.join("\n")}>
      {#if roots.length}
        <span class="path-label">Watching</span>
        <span class="path-value">{roots[0]}</span>
        {#if roots.length > 1}<span class="path-more">+{roots.length - 1}</span>{/if}
      {:else}
        <span class="path-label">No mods folder detected</span>
      {/if}
    </div>

    <button class="btn primary" onclick={runScan} disabled={scanning}>
      {scanning ? "Scanning…" : "Rescan"}
    </button>
  </header>

  {#if scanning}
    <div class="progress">
      <div class="bar" style="width: {pct}%"></div>
      <span class="progress-text tnum">{progress.done} / {progress.total}</span>
    </div>
  {/if}

  {#if errorMsg}
    <div class="error">{errorMsg}</div>
  {/if}

  <div class="statbar">
    <div class="stat">
      <span class="stat-num tnum">{mods.length}</span>
      <span class="stat-label">mods</span>
    </div>
    <div class="stat">
      <span class="stat-num tnum">{stats.maps}</span>
      <span class="stat-label">maps</span>
    </div>
    <div class="stat">
      <span class="stat-num tnum">{stats.scripts}</span>
      <span class="stat-label">script mods</span>
    </div>
    <div class="stat">
      <span class="stat-num tnum">{stats.unique}</span>
      <span class="stat-label">uniqueType</span>
    </div>
    <div class="stat" class:flag={stats.issues > 0}>
      <span class="stat-num tnum">{stats.issues}</span>
      <span class="stat-label">need attention</span>
    </div>
    {#if result}
      <div class="took tnum" title="Scan wall-clock time">
        scanned in {result.tookMs} ms
      </div>
    {/if}

    <input
      class="search"
      type="search"
      placeholder="Filter by title, author, or tech name…"
      bind:value={query}
    />
  </div>

  <div class="body">
    <CategoryRail
      items={mods}
      {selected}
      onSelect={(category, subcategory) => (selected = { category, subcategory })}
    />

    <main class="list">
      <div class="crumb">
        <span class="crumb-path">
          {#if selected.category}
            {selected.category}{selected.subcategory ? " › " + selected.subcategory : ""}
          {:else}
            All mods
          {/if}
        </span>
        <span class="crumb-count tnum">{filtered.length} shown</span>
      </div>

      <div class="list-body">
        {#if filtered.length === 0 && !scanning}
          <div class="empty">
            {mods.length === 0
              ? "No mods found yet. Point Silo at your mods folder and rescan."
              : "No mods match your filter."}
          </div>
        {:else}
          <VirtualList items={filtered} rowHeight={76}>
            {#snippet row(mod)}
              <ModRow {mod} />
            {/snippet}
          </VirtualList>
        {/if}
      </div>
    </main>
  </div>
</div>

<style>
  .app {
    display: flex;
    flex-direction: column;
    height: 100%;
  }
  .topbar {
    display: flex;
    align-items: center;
    gap: 20px;
    padding: 14px 20px;
    border-bottom: 1px solid var(--border);
    background: var(--surface);
  }
  .brand {
    display: flex;
    align-items: center;
    gap: 12px;
  }
  .logo {
    width: 40px;
    height: 40px;
    border-radius: var(--radius);
    display: grid;
    place-items: center;
    font-family: var(--font-display);
    font-weight: 600;
    font-size: 22px;
    color: var(--on-primary);
    background: linear-gradient(135deg, var(--green-500), var(--green-700));
    box-shadow: var(--shadow-1);
  }
  h1 {
    font-size: 20px;
    line-height: 1.1;
  }
  .tagline {
    margin: 0;
    font-size: 12px;
    color: var(--text-muted);
  }
  .path {
    flex: 1 1 auto;
    min-width: 0;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    overflow: hidden;
  }
  .path-label {
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
    flex: 0 0 auto;
  }
  .path-value {
    font-size: 12.5px;
    color: var(--text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .path-more {
    flex: 0 0 auto;
    font-size: 11px;
    color: var(--text-muted);
  }
  .btn {
    border: 1px solid var(--border);
    background: var(--surface-raised);
    color: var(--text);
    padding: 9px 18px;
    border-radius: var(--radius);
    font-weight: 600;
    transition: background 0.15s ease, border-color 0.15s ease;
  }
  .btn.primary {
    background: var(--primary);
    border-color: var(--primary);
    color: var(--on-primary);
  }
  .btn.primary:hover:not(:disabled) {
    background: var(--primary-hover);
  }
  .btn:disabled {
    opacity: 0.6;
    cursor: default;
  }
  .progress {
    position: relative;
    height: 4px;
    background: var(--border);
  }
  .bar {
    height: 100%;
    background: var(--accent);
    transition: width 0.1s linear;
  }
  .progress-text {
    position: absolute;
    right: 12px;
    top: 6px;
    font-size: 11px;
    color: var(--text-muted);
  }
  .error {
    padding: 10px 20px;
    background: color-mix(in srgb, var(--danger) 12%, var(--surface));
    color: var(--danger);
    font-size: 13px;
  }
  .statbar {
    display: flex;
    align-items: center;
    gap: 24px;
    padding: 12px 20px;
    border-bottom: 1px solid var(--border);
    background: var(--surface);
  }
  .stat {
    display: flex;
    align-items: baseline;
    gap: 6px;
  }
  .stat-num {
    font-family: var(--font-display);
    font-size: 18px;
    font-weight: 600;
  }
  .stat-label {
    font-size: 12px;
    color: var(--text-muted);
  }
  .stat.flag .stat-num {
    color: var(--warn);
  }
  .took {
    font-size: 11px;
    color: var(--text-muted);
    margin-left: auto;
  }
  .search {
    flex: 0 0 320px;
    padding: 9px 14px;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--bg);
    color: var(--text);
    font-family: inherit;
    font-size: 13px;
  }
  .search:focus {
    outline: 2px solid color-mix(in srgb, var(--accent) 55%, transparent);
    outline-offset: 1px;
  }
  .body {
    flex: 1 1 auto;
    min-height: 0;
    display: flex;
  }
  .list {
    flex: 1 1 auto;
    min-width: 0;
    min-height: 0;
    display: flex;
    flex-direction: column;
  }
  .crumb {
    flex: 0 0 auto;
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    padding: 8px 16px;
    border-bottom: 1px solid var(--border);
    background: var(--bg);
  }
  .crumb-path {
    font-family: var(--font-display);
    font-size: 14px;
    font-weight: 600;
  }
  .crumb-count {
    font-size: 12px;
    color: var(--text-muted);
  }
  .list-body {
    flex: 1 1 auto;
    min-height: 0;
  }
  .empty {
    display: grid;
    place-items: center;
    height: 100%;
    color: var(--text-muted);
    font-size: 14px;
    padding: 40px;
    text-align: center;
  }
</style>
