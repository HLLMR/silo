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
    getLoadouts,
    saveLoadout,
    deleteLoadout,
    exportLoadoutFile,
    importLoadoutFile,
    getSavegames,
    detectConflicts,
    detectGame,
    launchGame,
    modsWithSettings,
    getTags,
    setTags,
  } from "./lib/api";
  import type {
    ModEntry,
    ScanResult,
    CurationRow,
    ModInput,
    Loadout,
    Savegame,
    Conflict,
    GameInfo,
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
  import ModSettings from "./lib/components/ModSettings.svelte";
  import ModDetail from "./lib/components/ModDetail.svelte";

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
  // Auto-file newly-appeared mods into the archive on load (kept active). Persisted.
  let autoFileNew = $state(
    typeof localStorage !== "undefined"
      ? localStorage.getItem("silo.autoFile") !== "false"
      : true,
  );
  function setAutoFile(v: boolean) {
    autoFileNew = v;
    try {
      localStorage.setItem("silo.autoFile", String(v));
    } catch {}
  }

  // Silo only organizes .zip mods (the commercial distribution format). Unpacked
  // dir mods are left in place — that's how dev/work-in-progress mods live until
  // they're officially packaged.
  const isFileable = (m: ModEntry) => !m.organized && m.kind === "zip";
  const organizedCount = $derived(mods.filter((m) => m.organized).length);
  const unorganizedCount = $derived(mods.filter(isFileable).length);

  let loadouts = $state<Loadout[]>([]);
  let loadoutsOpen = $state(false);
  let savegames = $state<Savegame[]>([]);
  let savesOpen = $state(false);
  let conflicts = $state<Conflict[]>([]);
  let conflictsOpen = $state(false);
  let conflictTimer: ReturnType<typeof setTimeout> | undefined;
  let settingsOpen = $state(false);
  let healthOpen = $state(false);
  let gameInfo = $state<GameInfo | null>(null);
  let settingsModsSet = $state<Set<string>>(new Set());
  let settingsMod = $state<{ techName: string; title: string } | null>(null);

  async function launch() {
    if (
      criticalCount > 0 &&
      !confirm(
        `Your active set has ${criticalCount} critical conflict${criticalCount === 1 ? "" : "s"}. Launch anyway?`,
      )
    )
      return;
    try {
      await launchGame();
    } catch (e) {
      errorMsg = String(e);
    }
  }

  // Library health: missing dependencies, corrupt/unreadable mods, and mods the
  // game silently ignores (name starts with a digit).
  const health = $derived.by(() => {
    const lib = new Set(mods.map((m) => m.techName));
    const missingDeps: { mod: ModEntry; missing: string[] }[] = [];
    const corrupt: ModEntry[] = [];
    const ignored: ModEntry[] = [];
    for (const m of mods) {
      if (m.error) corrupt.push(m);
      if (m.ignoredDigitPrefix) ignored.push(m);
      const miss = m.dependencies.filter((d) => !lib.has(d));
      if (miss.length > 0) missingDeps.push({ mod: m, missing: miss });
    }
    return { missingDeps, corrupt, ignored };
  });
  const healthCount = $derived(
    health.missingDeps.length + health.corrupt.length + health.ignored.length,
  );

  const criticalCount = $derived(
    conflicts.filter((c) => c.severity === "critical").length,
  );

  async function runConflictCheck() {
    const active = mods.filter((m) => activeSet.has(m.techName));
    if (active.length < 2) {
      conflicts = [];
      return;
    }
    try {
      conflicts = await detectConflicts(
        active.map((m) => ({
          techName: m.techName,
          title: m.title,
          path: m.path,
          kind: m.kind,
        })),
      );
    } catch (e) {
      errorMsg = String(e);
    }
  }

  // Re-check conflicts (debounced) whenever the active set or library changes.
  $effect(() => {
    void activeSet;
    void mods;
    clearTimeout(conflictTimer);
    conflictTimer = setTimeout(runConflictCheck, 400);
  });

  const libraryTechNames = $derived(new Set(mods.map((m) => m.techName)));

  // Cross-reference a save's user mods (non-DLC) against the library.
  function saveStats(s: Savegame) {
    const userMods = s.mods.filter((m) => !m.isDlc);
    const present = userMods.filter((m) => libraryTechNames.has(m.modName));
    const missing = userMods.filter((m) => !libraryTechNames.has(m.modName));
    return { total: userMods.length, present, missing };
  }

  async function loadSavegames() {
    try {
      savegames = await getSavegames();
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function loadoutFromSave(s: Savegame) {
    const { present, missing } = saveStats(s);
    if (present.length === 0) {
      errorMsg = `None of “${s.name}”'s mods are in your library yet.`;
      return;
    }
    const note =
      missing.length > 0
        ? `\n\nNote: ${missing.length} mod(s) the save used aren't in your library and will be left out.`
        : "";
    if (
      !confirm(
        `Create a loadout “${s.name}” with ${present.length} mod(s) from this save?${note}`,
      )
    )
      return;
    try {
      await saveLoadout(null, s.name, present.map((m) => m.modName));
      await loadLoadouts();
      savesOpen = false;
      // Apply it immediately.
      const created = loadouts.find((l) => l.name === s.name);
      if (created) await applyLoadout(created);
    } catch (e) {
      errorMsg = String(e);
    }
  }

  // The loadout whose mod set exactly matches the current active set (if any).
  const activeLoadoutId = $derived.by(() => {
    for (const l of loadouts) {
      if (
        l.mods.length === activeSet.size &&
        l.mods.every((m) => activeSet.has(m))
      ) {
        return l.id;
      }
    }
    return null;
  });

  async function loadLoadouts() {
    try {
      loadouts = await getLoadouts();
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function applyLoadout(l: Loadout) {
    loadoutsOpen = false;
    busy = `Applying loadout “${l.name}”…`;
    activeSet = new Set(l.mods);
    try {
      await setActive(l.mods);
    } catch (e) {
      errorMsg = String(e);
    }
    busy = null;
  }

  async function saveCurrentLoadout() {
    const name = prompt("Name this loadout:", "");
    if (!name || !name.trim()) return;
    try {
      await saveLoadout(null, name.trim(), [...activeSet]);
      await loadLoadouts();
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function overwriteLoadout(l: Loadout) {
    try {
      await saveLoadout(l.id, l.name, [...activeSet]);
      await loadLoadouts();
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function removeLoadout(l: Loadout) {
    if (!confirm(`Delete loadout “${l.name}”? (Your mods aren't affected.)`)) return;
    try {
      await deleteLoadout(l.id);
      await loadLoadouts();
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function exportLoadout(l: Loadout) {
    try {
      await exportLoadoutFile(l.id, l.name);
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function importLoadout() {
    try {
      const id = await importLoadoutFile();
      if (id !== null) {
        await loadLoadouts();
        loadoutsOpen = true;
      }
    } catch (e) {
      errorMsg = String(e);
    }
  }

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

  // File loose (unorganized) mods into the archive. `keepActive` preserves their
  // loaded state (used by both the auto-filer and the manual button).
  async function fileLooseMods(keepActive: boolean) {
    const targets = effectiveMods.filter(isFileable);
    if (targets.length === 0) return;
    busy = `Filing ${targets.length} mod${targets.length > 1 ? "s" : ""} into the library…`;
    const inputs: ModInput[] = targets.map((m) => ({
      techName: m.techName,
      fileName: fileName(m.path),
      kind: m.kind,
      category: m.category,
      subcategory: m.subcategory,
    }));
    try {
      const rep = await applyOrganize(inputs);
      if (rep.errors.length) {
        errorMsg = rep.errors.slice(0, 3).join("; ");
      } else if (keepActive) {
        const next = new Set([...activeSet, ...targets.map((m) => m.techName)]);
        activeSet = next;
        await setActive([...next]);
      }
    } catch (e) {
      errorMsg = String(e);
    }
    busy = null;
    await runScan(false);
  }

  // Auto-filer: file new mods and keep them active (transparent to the game).
  const autoFile = () => fileLooseMods(true);
  // Manual "Organize N" button: file new mods, keep them active too.
  const organizeNew = () => fileLooseMods(true);

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
        rating: 0,
        note: null,
      }
    );
  }

  // Tags: techName -> string[]
  let tagMap = $state<Record<string, string[]>>({});
  let detailMod = $state<ModEntry | null>(null);
  let selectedTag = $state<string | null>(null);

  const allTags = $derived.by(() => {
    const s = new Set<string>();
    for (const arr of Object.values(tagMap)) for (const t of arr) s.add(t);
    return [...s].sort((a, b) => a.localeCompare(b));
  });

  function tagsOf(techName: string): string[] {
    return tagMap[techName] ?? [];
  }
  async function loadTags() {
    try {
      const rows = await getTags();
      const m: Record<string, string[]> = {};
      for (const r of rows) (m[r.techName] ??= []).push(r.tag);
      tagMap = m;
    } catch (e) {
      errorMsg = String(e);
    }
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
    if (selectedTag) {
      list = list.filter((m) => tagsOf(m.techName).includes(selectedTag!));
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

  type SortKey = "name" | "category" | "size" | "added" | "version";
  let sortBy = $state<SortKey>("name");
  let sortDir = $state<"asc" | "desc">("asc");

  const visible = $derived.by(() => {
    const arr = [...filtered];
    const mul = sortDir === "desc" ? -1 : 1;
    const name = (m: ModEntry) => (m.title ?? m.techName).toLowerCase();
    arr.sort((a, b) => {
      let r = 0;
      switch (sortBy) {
        case "name":
          r = name(a).localeCompare(name(b));
          break;
        case "size":
          r = a.size - b.size;
          break;
        case "added":
          r = a.mtimeMs - b.mtimeMs;
          break;
        case "version":
          r = (a.version ?? "").localeCompare(b.version ?? "", undefined, { numeric: true });
          break;
        case "category":
          r =
            a.category.localeCompare(b.category) ||
            (a.subcategory ?? "").localeCompare(b.subcategory ?? "") ||
            name(a).localeCompare(name(b));
          break;
      }
      return r * mul || name(a).localeCompare(name(b));
    });
    return arr;
  });

  // Select-all checkbox state over the filtered view.
  const filteredActiveCount = $derived(
    filtered.reduce((n, m) => n + (activeSet.has(m.techName) ? 1 : 0), 0),
  );
  const allFilteredActive = $derived(
    filtered.length > 0 && filteredActiveCount === filtered.length,
  );
  let selectAllEl = $state<HTMLInputElement>();
  $effect(() => {
    if (selectAllEl) {
      selectAllEl.indeterminate =
        filteredActiveCount > 0 && filteredActiveCount < filtered.length;
    }
  });

  // Bulk activate/deactivate the currently-filtered set (fast loadout building).
  async function setActiveForFiltered(active: boolean) {
    const next = new Set(activeSet);
    for (const m of filtered) {
      if (active) next.add(m.techName);
      else next.delete(m.techName);
    }
    activeSet = next;
    busy = active ? "Activating…" : "Deactivating…";
    try {
      await setActive([...next]);
    } catch (e) {
      errorMsg = String(e);
    }
    busy = null;
  }

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

  async function runScan(auto = true) {
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
    // Auto-file any mods still loose in the flat root (e.g. freshly downloaded),
    // keeping them active so filing is transparent to the game.
    if (auto && autoFileNew) {
      if (mods.some(isFileable)) {
        await autoFile();
      }
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
        await loadTags();
        await loadLoadouts();
        await loadSavegames();
        gameInfo = await detectGame();
        settingsModsSet = new Set(await modsWithSettings());
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

    <div class="topbar-spacer"></div>

    {#if savegames.length > 0}
      <button
        class="btn"
        class:on={savesOpen}
        onclick={() => (savesOpen = !savesOpen)}
        disabled={!!busy}
      >
        Savegames
      </button>
    {/if}

    <button
      class="btn loadout-btn"
      class:on={loadoutsOpen}
      onclick={() => (loadoutsOpen = !loadoutsOpen)}
      disabled={!!busy}
    >
      {#if activeLoadoutId !== null}
        ● {loadouts.find((l) => l.id === activeLoadoutId)?.name}
      {:else}
        Loadouts
      {/if}
    </button>

    {#if unorganizedCount > 0 && !autoFileNew}
      <button class="btn" onclick={organizeNew} disabled={!!busy || scanning}>
        Organize {unorganizedCount}
      </button>
    {/if}
    <button class="btn" onclick={() => runScan()} disabled={scanning || !!busy}>
      {scanning ? "Scanning…" : "Rescan"}
    </button>
    {#if gameInfo}
      <button
        class="btn primary launch-btn"
        title="Launch Farming Simulator 25 with the current active set"
        onclick={launch}
        disabled={!!busy}
      >
        ▶ Launch{activeSet.size ? ` (${activeSet.size})` : ""}
      </button>
    {/if}
    <button
      class="btn icon-btn"
      class:on={settingsOpen}
      title="Settings"
      aria-label="Settings"
      onclick={() => (settingsOpen = !settingsOpen)}
      disabled={!!busy}
    >
      ⚙
    </button>
  </header>

  {#if loadoutsOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div class="backdrop" onclick={() => (loadoutsOpen = false)}></div>
    <div class="loadouts-panel">
      <div class="lp-head">
        <span>Loadouts</span>
        <span class="lp-sub tnum">{activeSet.size} active</span>
      </div>
      {#if loadouts.length === 0}
        <div class="lp-empty">
          No loadouts yet. Activate the mods you want, then save them as a set.
        </div>
      {/if}
      {#each loadouts as l (l.id)}
        <div class="lp-row" class:active={l.id === activeLoadoutId}>
          <button class="lp-apply" onclick={() => applyLoadout(l)} title="Apply this loadout">
            <span class="lp-dot" class:on={l.id === activeLoadoutId}></span>
            <span class="lp-name">{l.name}</span>
            <span class="lp-count tnum">{l.mods.length}</span>
          </button>
          <button
            class="lp-icon"
            title="Overwrite with current active set"
            onclick={() => overwriteLoadout(l)}>⭯</button
          >
          <button class="lp-icon" title="Export to a .silo file" onclick={() => exportLoadout(l)}
            >⇪</button
          >
          <button class="lp-icon danger" title="Delete loadout" onclick={() => removeLoadout(l)}
            >✕</button
          >
        </div>
      {/each}
      <button class="lp-save" onclick={saveCurrentLoadout} disabled={activeSet.size === 0}>
        + Save current active set as a loadout
      </button>
      <button class="lp-import" onclick={importLoadout}>↧ Import a .silo file</button>
    </div>
  {/if}

  {#if savesOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div class="backdrop" onclick={() => (savesOpen = false)}></div>
    <div class="loadouts-panel saves">
      <div class="lp-head"><span>Savegames</span></div>
      {#each savegames as s (s.folder)}
        {@const st = saveStats(s)}
        <div class="sg-row">
          <div class="sg-info">
            <div class="sg-name">{s.name}</div>
            <div class="sg-meta">
              slot {s.index}{s.mapTitle ? ` · ${s.mapTitle}` : ""} ·
              <span class="tnum">{st.present.length}</span>/{st.total} mods in library{#if st.missing.length > 0}, <span
                  class="sg-missing tnum">{st.missing.length} missing</span
                >{/if}
            </div>
          </div>
          <button
            class="sg-make"
            title={st.missing.length
              ? `Build a loadout from the ${st.present.length} mods you have (${st.missing.length} missing)`
              : "Build a loadout from this save's mods"}
            onclick={() => loadoutFromSave(s)}
            disabled={st.present.length === 0}
          >
            → Loadout
          </button>
        </div>
      {/each}
    </div>
  {/if}

  {#if conflictsOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div class="backdrop" onclick={() => (conflictsOpen = false)}></div>
    <div class="conflicts-panel">
      <div class="lp-head">
        <span>Conflicts in the active set</span>
        <span class="lp-sub tnum">{criticalCount} critical</span>
      </div>
      {#if conflicts.length === 0}
        <div class="lp-empty">
          No conflicts in the {activeSet.size} active mod(s). Activate more and Silo re-checks automatically.
        </div>
      {/if}
      {#each conflicts as c (c.severity + c.kind + c.name)}
        <div class="cf-row" class:crit={c.severity === "critical"} class:info={c.severity === "info"}>
          <div class="cf-top">
            <span class="cf-sev">{c.severity}</span>
            <span class="cf-kind">{c.kind}</span>
            <span class="cf-name">{c.name}</span>
          </div>
          <div class="cf-mods">{c.mods.join("  ·  ")}</div>
          <div class="cf-why">{c.explanation}</div>
        </div>
      {/each}
    </div>
  {/if}

  {#if settingsOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div class="backdrop" onclick={() => (settingsOpen = false)}></div>
    <div class="loadouts-panel settings">
      <div class="lp-head"><span>Settings</span></div>

      <div class="set-section">
        <div class="set-label">Mods folder</div>
        {#if roots.length}
          {#each roots as r (r)}
            <div class="set-path">{r}</div>
          {/each}
        {:else}
          <div class="set-path muted">No mods folder detected.</div>
        {/if}
      </div>

      <div class="set-section">
        <div class="set-label">Game</div>
        {#if gameInfo}
          <div class="set-path">{gameInfo.installDir}</div>
        {:else}
          <div class="set-path muted">
            Farming Simulator 25 install not found. The Launch button is hidden.
          </div>
        {/if}
      </div>

      <div class="set-section">
        <div class="set-row">
          <div>
            <div class="set-label">Auto-file new mods</div>
            <div class="set-hint">
              On load, move newly-downloaded .zip mods into the library and keep them active.
            </div>
          </div>
          <button
            class="switch"
            class:on={autoFileNew}
            role="switch"
            aria-checked={autoFileNew}
            aria-label="Auto-file new mods"
            onclick={() => setAutoFile(!autoFileNew)}
          >
            <span class="knob"></span>
          </button>
        </div>
      </div>

      {#if organizedCount > 0}
        <div class="set-section">
          <div class="set-label">Library layout</div>
          <div class="set-hint">
            {organizedCount} mod(s) organized in <code>mods/archive/</code>. Restore moves them all
            back to a vanilla flat folder (your mods aren't deleted).
          </div>
          <button
            class="set-danger"
            onclick={() => {
              settingsOpen = false;
              restoreVanilla();
            }}
            disabled={!!busy || scanning}
          >
            Restore vanilla layout
          </button>
        </div>
      {/if}
    </div>
  {/if}

  {#if settingsMod}
    <ModSettings
      modName={settingsMod.techName}
      title={settingsMod.title}
      onClose={() => (settingsMod = null)}
    />
  {/if}

  {#if detailMod}
    {@const dm = detailMod}
    <ModDetail
      mod={dm}
      curation={cur(dm.techName)}
      tags={tagsOf(dm.techName)}
      active={activeSet.has(dm.techName)}
      organized={dm.organized}
      hasSettings={settingsModsSet.has(dm.techName)}
      {libraryTechNames}
      {conflicts}
      onClose={() => (detailMod = null)}
      onToggle={(flag) => toggleCuration(dm.techName, flag)}
      onToggleActive={() => toggleActive(dm.techName)}
      onOpenSettings={() =>
        (settingsMod = { techName: dm.techName, title: dm.title ?? dm.techName })}
      onCurationChange={(row) => (curationMap = { ...curationMap, [dm.techName]: row })}
      onTagsChange={(t) => (tagMap = { ...tagMap, [dm.techName]: t })}
      onFilterTag={(t) => {
        selectedTag = t;
        detailMod = null;
      }}
    />
  {/if}

  {#if healthOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div class="backdrop" onclick={() => (healthOpen = false)}></div>
    <div class="conflicts-panel">
      <div class="lp-head">
        <span>Library health</span>
        <span class="lp-sub tnum">{healthCount} issue{healthCount === 1 ? "" : "s"}</span>
      </div>

      {#if healthCount === 0}
        <div class="lp-empty">Everything looks healthy — no problems found.</div>
      {/if}

      {#if health.missingDeps.length > 0}
        <div class="hz-group">Missing dependencies ({health.missingDeps.length})</div>
        {#each health.missingDeps as d (d.mod.techName)}
          <div class="hz-row">
            <div class="hz-name">{d.mod.title ?? d.mod.techName}</div>
            <div class="hz-detail">
              needs {#each d.missing as dep, i (dep)}<span class="hz-dep">{dep}</span>{#if i < d.missing.length - 1}, {/if}{/each}
              — not in your library
            </div>
          </div>
        {/each}
      {/if}

      {#if health.ignored.length > 0}
        <div class="hz-group">Ignored by the game — name starts with a digit ({health.ignored.length})</div>
        {#each health.ignored as m (m.techName)}
          <div class="hz-row">
            <div class="hz-name">{m.title ?? m.techName}</div>
            <div class="hz-detail">
              <span class="tnum">{m.techName}</span> — FS won't load a mod whose name starts with a number.
            </div>
          </div>
        {/each}
      {/if}

      {#if health.corrupt.length > 0}
        <div class="hz-group">Corrupt / unreadable ({health.corrupt.length})</div>
        {#each health.corrupt as m (m.techName)}
          <div class="hz-row">
            <div class="hz-name">{m.title ?? m.techName}</div>
            <div class="hz-detail">{m.error}</div>
          </div>
        {/each}
      {/if}
    </div>
  {/if}

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
    <button
      class="stat statbtn"
      class:flag={conflicts.length > 0}
      class:crit={criticalCount > 0}
      title="Conflicts within the active set"
      onclick={() => (conflictsOpen = !conflictsOpen)}
    >
      <span class="stat-num tnum">{conflicts.length}</span>
      <span class="stat-label">conflict{conflicts.length === 1 ? "" : "s"}</span>
    </button>
    <button
      class="stat statbtn"
      class:flag={healthCount > 0}
      title="Library health: missing dependencies, corrupt mods, ignored names"
      onclick={() => (healthOpen = !healthOpen)}
    >
      <span class="stat-num tnum">{healthCount}</span>
      <span class="stat-label">need attention</span>
    </button>
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
        <input
          type="checkbox"
          class="select-all"
          bind:this={selectAllEl}
          checked={allFilteredActive}
          disabled={!!busy || filtered.length === 0}
          title="Activate / deactivate everything in this view"
          onchange={(e) => setActiveForFiltered(e.currentTarget.checked)}
        />
        <span class="crumb-path">
          {#if selected.category}
            {selected.category}{selected.subcategory ? " › " + selected.subcategory : ""}
          {:else}
            All mods
          {/if}
        </span>
        {#if selectedTag}
          <button class="crumb-tag" onclick={() => (selectedTag = null)} title="Clear tag filter">
            #{selectedTag} ✕
          </button>
        {/if}
        <span class="crumb-count tnum">{filtered.length} shown</span>

        <div class="tb-spacer"></div>

        <div class="tb-group">
          <label class="tb-sort">
            Sort
            <select bind:value={sortBy}>
              <option value="name">Name</option>
              <option value="category">Category</option>
              <option value="size">Size</option>
              <option value="added">Recently added</option>
              <option value="version">Version</option>
            </select>
          </label>
          <button
            class="tb-dir"
            title={sortDir === "asc" ? "Ascending" : "Descending"}
            onclick={() => (sortDir = sortDir === "asc" ? "desc" : "asc")}
          >
            {sortDir === "asc" ? "↑" : "↓"}
          </button>
        </div>
      </div>

      <div class="list-body">
        {#if filtered.length === 0 && !scanning}
          <div class="empty">
            {mods.length === 0
              ? "No mods found yet. Point Silo at your mods folder and rescan."
              : "No mods match your filter."}
          </div>
        {:else}
          <VirtualList items={visible} rowHeight={76}>
            {#snippet row(mod)}
              <ModRow
                {mod}
                curation={cur(mod.techName)}
                overridden={!!overrideMap[mod.techName]}
                organized={mod.organized}
                active={activeSet.has(mod.techName)}
                hasSettings={settingsModsSet.has(mod.techName)}
                tags={tagsOf(mod.techName)}
                onToggle={(flag) => toggleCuration(mod.techName, flag)}
                onToggleActive={() => toggleActive(mod.techName)}
                onEditCategory={(ev) => openEditor(mod.techName, ev)}
                onOpenSettings={() =>
                  (settingsMod = { techName: mod.techName, title: mod.title ?? mod.techName })}
                onOpenDetail={() => (detailMod = mod)}
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
  .topbar-spacer {
    flex: 1 1 auto;
  }
  .icon-btn {
    padding: 9px 12px;
    font-size: 16px;
    line-height: 1;
  }
  .icon-btn.on {
    color: var(--primary);
    border-color: color-mix(in srgb, var(--primary) 45%, var(--border));
  }
  .launch-btn {
    font-weight: 700;
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
  .loadout-btn.on {
    border-color: color-mix(in srgb, var(--primary) 50%, var(--border));
    color: var(--primary);
  }
  .loadouts-panel {
    position: fixed;
    z-index: 50;
    top: 66px;
    right: 20px;
    width: 320px;
    max-height: 70vh;
    overflow-y: auto;
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    box-shadow: var(--shadow-2);
    padding: 8px;
    scrollbar-width: thin;
  }
  .lp-head {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    padding: 6px 8px 10px;
    font-family: var(--font-display);
    font-weight: 600;
  }
  .lp-sub {
    font-size: 11.5px;
    color: var(--text-muted);
    font-family: var(--font-ui);
  }
  .lp-empty {
    padding: 10px 8px 14px;
    font-size: 12.5px;
    color: var(--text-muted);
    line-height: 1.5;
  }
  .lp-row {
    display: flex;
    align-items: center;
    gap: 2px;
    border-radius: var(--radius-sm);
  }
  .lp-row.active {
    background: color-mix(in srgb, var(--primary) 12%, transparent);
  }
  .lp-apply {
    flex: 1 1 auto;
    display: flex;
    align-items: center;
    gap: 9px;
    min-width: 0;
    border: none;
    background: transparent;
    color: var(--text);
    padding: 9px 10px;
    border-radius: var(--radius-sm);
    font-size: 13px;
  }
  .lp-apply:hover {
    background: color-mix(in srgb, var(--primary) 10%, transparent);
  }
  .lp-dot {
    flex: 0 0 auto;
    width: 9px;
    height: 9px;
    border-radius: 50%;
    border: 2px solid var(--border);
  }
  .lp-dot.on {
    background: var(--primary);
    border-color: var(--primary);
  }
  .lp-name {
    flex: 1 1 auto;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    text-align: left;
    font-weight: 600;
  }
  .lp-count {
    flex: 0 0 auto;
    font-size: 11.5px;
    color: var(--text-muted);
  }
  .lp-icon {
    flex: 0 0 auto;
    border: none;
    background: transparent;
    color: var(--text-muted);
    width: 28px;
    height: 30px;
    border-radius: var(--radius-sm);
    font-size: 14px;
  }
  .lp-icon:hover {
    background: color-mix(in srgb, var(--primary) 12%, transparent);
    color: var(--text);
  }
  .lp-icon.danger:hover {
    background: color-mix(in srgb, var(--danger) 14%, transparent);
    color: var(--danger);
  }
  .lp-save {
    display: block;
    width: 100%;
    margin-top: 6px;
    border: 1px dashed var(--border);
    background: transparent;
    color: var(--primary);
    padding: 10px;
    border-radius: var(--radius-sm);
    font-size: 12.5px;
    font-weight: 600;
  }
  .lp-save:hover:not(:disabled) {
    background: color-mix(in srgb, var(--primary) 10%, transparent);
  }
  .lp-save:disabled {
    opacity: 0.5;
    cursor: default;
  }
  .lp-import {
    display: block;
    width: 100%;
    margin-top: 6px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    padding: 8px;
    border-radius: var(--radius-sm);
    font-size: 12px;
    font-weight: 600;
  }
  .lp-import:hover {
    background: color-mix(in srgb, var(--primary) 8%, transparent);
    color: var(--text);
  }
  .loadouts-panel.saves {
    width: 380px;
  }
  .loadouts-panel.settings {
    width: 380px;
  }
  .set-section {
    padding: 10px 8px;
    border-top: 1px solid var(--border);
  }
  .set-section:first-of-type {
    border-top: none;
  }
  .set-label {
    font-size: 12px;
    font-weight: 600;
    color: var(--text);
    margin-bottom: 6px;
  }
  .set-hint {
    font-size: 12px;
    color: var(--text-muted);
    line-height: 1.5;
    margin-bottom: 8px;
  }
  .set-path {
    font-size: 12px;
    color: var(--text-muted);
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    padding: 7px 9px;
    word-break: break-all;
    margin-bottom: 4px;
  }
  .set-path.muted {
    font-style: italic;
  }
  .set-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
  }
  .switch {
    flex: 0 0 auto;
    width: 40px;
    height: 23px;
    border-radius: 999px;
    border: 1px solid var(--border);
    background: var(--bg);
    position: relative;
    transition: background 0.15s ease, border-color 0.15s ease;
  }
  .switch .knob {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 17px;
    height: 17px;
    border-radius: 50%;
    background: var(--text-muted);
    transition: transform 0.15s ease, background 0.15s ease;
  }
  .switch.on {
    background: color-mix(in srgb, var(--primary) 30%, transparent);
    border-color: var(--primary);
  }
  .switch.on .knob {
    transform: translateX(17px);
    background: var(--primary);
  }
  .set-danger {
    border: 1px solid color-mix(in srgb, var(--danger) 45%, var(--border));
    background: transparent;
    color: var(--danger);
    padding: 9px 12px;
    border-radius: var(--radius-sm);
    font-size: 12.5px;
    font-weight: 600;
    width: 100%;
  }
  .set-danger:hover:not(:disabled) {
    background: color-mix(in srgb, var(--danger) 10%, transparent);
  }
  .set-danger:disabled {
    opacity: 0.5;
    cursor: default;
  }
  .sg-row {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px;
    border-radius: var(--radius-sm);
  }
  .sg-row:hover {
    background: color-mix(in srgb, var(--primary) 8%, transparent);
  }
  .sg-info {
    flex: 1 1 auto;
    min-width: 0;
  }
  .sg-name {
    font-weight: 600;
    font-size: 13px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .sg-meta {
    font-size: 11.5px;
    color: var(--text-muted);
    margin-top: 2px;
  }
  .sg-missing {
    color: var(--warn);
  }
  .sg-make {
    flex: 0 0 auto;
    border: 1px solid var(--border);
    background: var(--bg);
    color: var(--primary);
    padding: 7px 12px;
    border-radius: var(--radius-sm);
    font-size: 12.5px;
    font-weight: 600;
  }
  .sg-make:hover:not(:disabled) {
    background: color-mix(in srgb, var(--primary) 12%, transparent);
  }
  .sg-make:disabled {
    opacity: 0.5;
    cursor: default;
    color: var(--text-muted);
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
  .statbtn {
    border: none;
    background: transparent;
    cursor: pointer;
    padding: 0;
    font: inherit;
  }
  .statbtn:hover {
    opacity: 0.8;
  }
  .statbtn.flag .stat-num {
    color: var(--warn);
  }
  .statbtn.crit .stat-num {
    color: var(--danger);
  }
  .conflicts-panel {
    position: fixed;
    z-index: 50;
    top: 120px;
    left: 50%;
    transform: translateX(-50%);
    width: 560px;
    max-width: calc(100vw - 40px);
    max-height: 70vh;
    overflow-y: auto;
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    box-shadow: var(--shadow-2);
    padding: 10px;
    scrollbar-width: thin;
  }
  .cf-row {
    padding: 10px 10px 12px;
    border-radius: var(--radius-sm);
    border-left: 3px solid var(--warn);
    background: color-mix(in srgb, var(--warn) 6%, transparent);
    margin-bottom: 8px;
  }
  .cf-row.crit {
    border-left-color: var(--danger);
    background: color-mix(in srgb, var(--danger) 6%, transparent);
  }
  .cf-row.info {
    border-left-color: var(--info);
    background: color-mix(in srgb, var(--info) 5%, transparent);
  }
  .cf-row.info .cf-sev {
    color: var(--info);
  }
  .cf-top {
    display: flex;
    align-items: baseline;
    gap: 8px;
  }
  .cf-sev {
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--warn);
  }
  .cf-row.crit .cf-sev {
    color: var(--danger);
  }
  .cf-kind {
    font-size: 11px;
    color: var(--text-muted);
  }
  .cf-name {
    font-weight: 600;
    font-family: var(--font-display);
  }
  .cf-mods {
    margin-top: 5px;
    font-size: 12.5px;
    color: var(--text);
  }
  .cf-why {
    margin-top: 5px;
    font-size: 12px;
    color: var(--text-muted);
    line-height: 1.5;
  }
  .hz-group {
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--text-muted);
    padding: 12px 8px 6px;
  }
  .hz-row {
    padding: 7px 10px;
    border-radius: var(--radius-sm);
  }
  .hz-row:hover {
    background: color-mix(in srgb, var(--primary) 6%, transparent);
  }
  .hz-name {
    font-weight: 600;
    font-size: 13px;
  }
  .hz-detail {
    font-size: 12px;
    color: var(--text-muted);
    margin-top: 2px;
  }
  .hz-dep {
    color: var(--warn);
    font-weight: 600;
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
    align-items: center;
    gap: 12px;
    padding: 7px 16px;
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
  .crumb-tag {
    border: 1px solid color-mix(in srgb, var(--info) 40%, var(--border));
    background: color-mix(in srgb, var(--info) 12%, transparent);
    color: var(--info);
    border-radius: 999px;
    padding: 3px 10px;
    font-size: 12px;
    font-weight: 600;
  }
  .select-all {
    flex: 0 0 auto;
    width: 15px;
    height: 15px;
    margin: 0 7px 0 0;
    accent-color: var(--primary);
    cursor: pointer;
  }
  .tb-spacer {
    flex: 1 1 auto;
  }
  .tb-group {
    display: flex;
    align-items: center;
    gap: 4px;
  }
  .tb-sort {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    color: var(--text-muted);
  }
  .tb-sort select {
    font-family: inherit;
    font-size: 12.5px;
    color: var(--text);
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    padding: 6px 8px;
  }
  .tb-dir {
    border: 1px solid var(--border);
    background: var(--surface);
    color: var(--text);
    width: 30px;
    height: 30px;
    border-radius: var(--radius-sm);
    font-size: 14px;
  }
  .tb-dir:hover {
    border-color: color-mix(in srgb, var(--primary) 45%, var(--border));
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
