<script lang="ts">
  import { onMount } from "svelte";
  import {
    defaultModsPaths,
    scanMods,
    onScanProgress,
    getCuration,
    setCuration,
    getOverrides,
    setOverride,
    applyOrganize,
    setActive,
    flatten,
  } from "./lib/api";
  import type {
    ModEntry,
    ScanResult,
    CurationRow,
    ModInput,
  } from "./lib/types";

  const CATEGORIES = [
    "Maps",
    "Tractors",
    "Harvesters",
    "Implements",
    "Cars & Trucks",
    "Vehicles",
    "Placeables",
    "Objects",
    "Decorations",
    "Textures",
    "Sounds",
    "Realism",
    "Cheats",
    "Scripts & Tools",
    "Other",
  ];
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
  let curationMap = $state<Record<string, CurationRow>>({});
  let overrideMap = $state<
    Record<string, { category: string; subcategory: string | null }>
  >({});
  let showHidden = $state(false);
  let favoritesOnly = $state(false);
  let editing = $state<{ techName: string; x: number; y: number } | null>(null);
  let activeSet = $state<Set<string>>(new Set());
  let busy = $state<string | null>(null);

  const organizedCount = $derived(mods.filter((m) => m.organized).length);
  const unorganizedCount = $derived(mods.filter((m) => !m.organized).length);

  function fileName(path: string): string {
    return path.split(/[\\/]/).pop() ?? path;
  }

  async function toggleActive(techName: string) {
    const next = new Set(activeSet);
    if (next.has(techName)) next.delete(techName);
    else next.add(techName);
    activeSet = next;
    try {
      await setActive([...next]);
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function organizeNew() {
    const targets = effectiveMods.filter((m) => !m.organized);
    if (targets.length === 0) return;
    busy = `Organizing ${targets.length} mods…`;
    const inputs: ModInput[] = targets.map((m) => ({
      techName: m.techName,
      fileName: fileName(m.path),
      kind: m.kind,
      category: m.category,
      subcategory: m.subcategory,
    }));
    try {
      const rep = await applyOrganize(inputs);
      if (rep.errors.length) errorMsg = rep.errors.slice(0, 3).join("; ");
    } catch (e) {
      errorMsg = String(e);
    }
    busy = null;
    await runScan();
  }

  async function restoreVanilla() {
    if (
      !confirm(
        "Restore a vanilla flat mods/ folder?\n\nThis moves every mod back out of archive/ and removes all links. Your mods are not deleted — this just undoes Silo's organization.",
      )
    )
      return;
    busy = "Restoring vanilla layout…";
    try {
      const rep = await flatten();
      if (rep.errors.length) errorMsg = rep.errors.slice(0, 3).join("; ");
    } catch (e) {
      errorMsg = String(e);
    }
    busy = null;
    await runScan();
  }

  // Overrides applied as a display layer over the scanned category.
  const effectiveMods = $derived(
    mods.map((m) => {
      const o = overrideMap[m.techName];
      return o ? { ...m, category: o.category, subcategory: o.subcategory } : m;
    }),
  );

  function openEditor(techName: string, ev: MouseEvent) {
    ev.stopPropagation();
    editing = { techName, x: ev.clientX, y: ev.clientY };
  }

  async function setCategory(techName: string, category: string) {
    overrideMap = {
      ...overrideMap,
      [techName]: { category, subcategory: null },
    };
    editing = null;
    try {
      await setOverride({ techName, category, subcategory: null });
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function resetCategory(techName: string) {
    const next = { ...overrideMap };
    delete next[techName];
    overrideMap = next;
    editing = null;
    try {
      await setOverride({ techName, category: "", subcategory: null });
    } catch (e) {
      errorMsg = String(e);
    }
  }

  function cur(techName: string): CurationRow {
    return (
      curationMap[techName] ?? {
        techName,
        favorite: false,
        hidden: false,
        broken: false,
        note: null,
      }
    );
  }

  async function toggleCuration(
    techName: string,
    flag: "favorite" | "hidden" | "broken",
  ) {
    const c = cur(techName);
    const next: CurationRow = { ...c, [flag]: !c[flag] };
    curationMap = { ...curationMap, [techName]: next };
    try {
      await setCuration(next);
    } catch (e) {
      errorMsg = String(e);
    }
  }

  const q = $derived(query.trim().toLowerCase());
  const filtered = $derived.by(() => {
    let list = effectiveMods;
    if (selected.category) {
      list = list.filter(
        (m) =>
          m.category === selected.category &&
          (!selected.subcategory || m.subcategory === selected.subcategory),
      );
    }
    if (!showHidden) {
      list = list.filter((m) => !cur(m.techName).hidden);
    }
    if (favoritesOnly) {
      list = list.filter((m) => cur(m.techName).favorite);
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
      activeSet = new Set(r.mods.filter((m) => m.active).map((m) => m.techName));
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
        const rows = await getCuration();
        curationMap = Object.fromEntries(rows.map((r) => [r.techName, r]));
        const ovs = await getOverrides();
        overrideMap = Object.fromEntries(
          ovs.map((o) => [o.techName, { category: o.category, subcategory: o.subcategory }]),
        );
      } catch (e) {
        errorMsg = String(e);
      }
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

    {#if unorganizedCount > 0}
      <button class="btn" onclick={organizeNew} disabled={!!busy || scanning}>
        Organize {unorganizedCount}
      </button>
    {/if}
    {#if organizedCount > 0}
      <button class="btn subtle" onclick={restoreVanilla} disabled={!!busy || scanning}>
        Restore vanilla
      </button>
    {/if}
    <button class="btn primary" onclick={runScan} disabled={scanning || !!busy}>
      {scanning ? "Scanning…" : "Rescan"}
    </button>
  </header>

  {#if scanning}
    <div class="progress">
      <div class="bar" style="width: {pct}%"></div>
      <span class="progress-text tnum">{progress.done} / {progress.total}</span>
    </div>
  {/if}

  {#if busy}
    <div class="busy">{busy}</div>
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
    <div class="stat">
      <span class="stat-num tnum">{activeSet.size}</span>
      <span class="stat-label">active</span>
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

    <button
      class="toggle"
      class:on={favoritesOnly}
      title="Show favorites only"
      onclick={() => (favoritesOnly = !favoritesOnly)}
    >
      {favoritesOnly ? "★" : "☆"} Favorites
    </button>
    <button
      class="toggle"
      class:on={showHidden}
      title="Show hidden mods"
      onclick={() => (showHidden = !showHidden)}
    >
      Hidden
    </button>

    <input
      class="search"
      type="search"
      placeholder="Filter by title, author, or tech name…"
      bind:value={query}
    />
  </div>

  <div class="body">
    <CategoryRail
      items={effectiveMods}
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
              <ModRow
                {mod}
                curation={cur(mod.techName)}
                overridden={!!overrideMap[mod.techName]}
                organized={mod.organized}
                active={activeSet.has(mod.techName)}
                onToggle={(flag) => toggleCuration(mod.techName, flag)}
                onToggleActive={() => toggleActive(mod.techName)}
                onEditCategory={(ev) => openEditor(mod.techName, ev)}
              />
            {/snippet}
          </VirtualList>
        {/if}
      </div>
    </main>
  </div>

  {#if editing}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div class="backdrop" onclick={() => (editing = null)}></div>
    <div
      class="menu"
      style="left: {Math.min(editing.x, window.innerWidth - 220)}px; top: {Math.min(
        editing.y,
        window.innerHeight - 420,
      )}px"
    >
      <div class="menu-head">Set category</div>
      {#each CATEGORIES as c (c)}
        <button class="menu-item" onclick={() => setCategory(editing!.techName, c)}>
          {c}
        </button>
      {/each}
      <button class="menu-item reset" onclick={() => resetCategory(editing!.techName)}>
        ↺ Reset to auto
      </button>
    </div>
  {/if}
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
  .btn.subtle {
    color: var(--text-muted);
  }
  .btn:hover:not(:disabled):not(.primary) {
    color: var(--text);
    border-color: color-mix(in srgb, var(--primary) 40%, var(--border));
  }
  .btn:disabled {
    opacity: 0.6;
    cursor: default;
  }
  .busy {
    padding: 8px 20px;
    background: color-mix(in srgb, var(--accent) 14%, var(--surface));
    color: var(--gold-700);
    font-size: 13px;
    font-weight: 600;
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
  .toggle {
    flex: 0 0 auto;
    border: 1px solid var(--border);
    background: var(--bg);
    color: var(--text-muted);
    padding: 8px 12px;
    border-radius: var(--radius);
    font-size: 12.5px;
    font-weight: 600;
  }
  .toggle:hover {
    color: var(--text);
  }
  .toggle.on {
    background: color-mix(in srgb, var(--accent) 16%, transparent);
    border-color: color-mix(in srgb, var(--accent) 45%, var(--border));
    color: var(--accent);
  }
  .search {
    flex: 0 0 280px;
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
  .backdrop {
    position: fixed;
    inset: 0;
    z-index: 40;
  }
  .menu {
    position: fixed;
    z-index: 50;
    width: 208px;
    max-height: 400px;
    overflow-y: auto;
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    box-shadow: var(--shadow-2);
    padding: 6px;
    scrollbar-width: thin;
  }
  .menu-head {
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
    padding: 6px 10px;
  }
  .menu-item {
    display: block;
    width: 100%;
    text-align: left;
    border: none;
    background: transparent;
    color: var(--text);
    padding: 8px 10px;
    border-radius: var(--radius-sm);
    font-size: 13px;
  }
  .menu-item:hover {
    background: color-mix(in srgb, var(--primary) 14%, transparent);
    color: var(--primary);
  }
  .menu-item.reset {
    margin-top: 4px;
    border-top: 1px solid var(--border);
    border-radius: 0;
    color: var(--text-muted);
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
