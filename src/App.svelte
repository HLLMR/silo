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
    planOrganize,
    applyOrganize,
    setActive,
    flatten,
    clearScanCache,
    getLoadouts,
    saveLoadout,
    deleteLoadout,
    exportLoadoutFile,
    importLoadoutFile,
    getSavegames,
    backupSavegame,
    detectConflicts,
    detectGame,
    launchGame,
    modsWithSettings,
    getTags,
    setTags,
    getModRepos,
    checkModUpdate,
    catalogCheckUpdates,
    downloadUpdate,
    saveTextFile,
    userDirPath,
    bisectSnapshotGet,
    bisectSnapshotClear,
  } from "./lib/api";
  import {
    GAME_GRAPHICS_FIELDS,
    GAME_GRAPHICS_PRESETS,
    SAVEGAME_FIELDS,
    SAVEGAME_PRESETS,
    type CfgField,
    type CfgPreset,
  } from "./lib/configSchemas";
  import type {
    ModEntry,
    ScanResult,
    CurationRow,
    ModInput,
    PlannedMove,
    Loadout,
    Savegame,
    Conflict,
    GameInfo,
    UpdateRow,
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
  import ConfigEditor from "./lib/components/ConfigEditor.svelte";
  import ModBrowser from "./lib/components/ModBrowser.svelte";
  import LogTriage from "./lib/components/LogTriage.svelte";
  import BindingsView from "./lib/components/BindingsView.svelte";
  import MpSync from "./lib/components/MpSync.svelte";
  import BridgeTool from "./lib/components/BridgeTool.svelte";
  import ConflictsPanel from "./lib/components/panels/ConflictsPanel.svelte";
  import HealthPanel from "./lib/components/panels/HealthPanel.svelte";
  import StatsPanel from "./lib/components/panels/StatsPanel.svelte";
  import UpdatesPanel from "./lib/components/panels/UpdatesPanel.svelte";
  import LoadoutsPanel from "./lib/components/panels/LoadoutsPanel.svelte";
  import SavegamesPanel from "./lib/components/panels/SavegamesPanel.svelte";
  import SettingsPanel from "./lib/components/panels/SettingsPanel.svelte";
  import CategoryMenu from "./lib/components/panels/CategoryMenu.svelte";
  import Topbar from "./lib/components/panels/Topbar.svelte";
  import StatBar from "./lib/components/panels/StatBar.svelte";
  import LibraryToolbar from "./lib/components/panels/LibraryToolbar.svelte";
  import OrganizePreview from "./lib/components/OrganizePreview.svelte";
  import { fmtSize } from "./lib/format";

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
  let flaggedOnly = $state(false);
  let conflictedOnly = $state(false);
  let editing = $state<{ techName: string; x: number; y: number } | null>(null);
  let activeSet = $state<Set<string>>(new Set());
  let busy = $state<string | null>(null);
  // Auto-file newly-appeared mods into the archive on load (kept active). OPT-IN:
  // defaults OFF so a first run never moves the user's files without them choosing to.
  // Enable it via Settings, or use the explicit "Organize" button. Persisted.
  let autoFileNew = $state(
    typeof localStorage !== "undefined"
      ? localStorage.getItem("silo.autoFile") === "true"
      : false,
  );
  function setAutoFile(v: boolean) {
    autoFileNew = v;
    try {
      localStorage.setItem("silo.autoFile", String(v));
    } catch {}
  }

  // Appearance: system (follow OS) / light / dark. Persisted; applied to <html>.
  let theme = $state<"system" | "light" | "dark">(
    (typeof localStorage !== "undefined"
      ? (localStorage.getItem("silo.theme") as "system" | "light" | "dark" | null)
      : null) ?? "system",
  );
  function setTheme(t: "system" | "light" | "dark") {
    theme = t;
    try {
      localStorage.setItem("silo.theme", t);
    } catch {}
  }
  $effect(() => {
    const el = document.documentElement;
    if (theme === "system") el.removeAttribute("data-theme");
    else el.setAttribute("data-theme", theme);
  });

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
  let statsOpen = $state(false);
  let logOpen = $state(false);
  let bindingsOpen = $state(false);
  let mpOpen = $state(false);
  let bridgeOpen = $state(false);
  // Active set as manifest refs for MP sync (techName, path, kind, version).
  const activeModRefs = $derived(
    mods
      .filter((m) => activeSet.has(m.techName))
      .map((m) => ({ techName: m.techName, path: m.path, kind: m.kind, version: m.version })),
  );
  // Set when a previous bisection didn't finish (app closed mid-run) — the user's mod
  // set may be partially applied, so offer to restore it.
  let bisectRecovery = $state<string[] | null>(null);
  // Top-level view: the local library, or the remote catalog browser.
  let view = $state<"library" | "browse">("library");
  // Measured so the fixed detail drawers can sit *below* the header instead of over it
  // (the topbar wraps, so its height isn't constant).
  let topbarH = $state(56);

  /** Switch views and close the Library drawer so it doesn't hang over Browse. The
   *  Browse drawer lives inside ModBrowser, which unmounts on switch, so it self-closes. */
  function switchView(v: "library" | "browse") {
    if (view === v) return;
    view = v;
    detailMod = null;
  }
  let userDir = $state<string | null>(null);
  let configEditor = $state<{
    title: string;
    path: string;
    fields: CfgField[];
    presets: CfgPreset[];
    footnote?: string;
  } | null>(null);

  function openGameGraphics() {
    if (!userDir) return;
    settingsOpen = false;
    configEditor = {
      title: "Graphics settings — game.xml",
      path: `${userDir}/game.xml`,
      fields: GAME_GRAPHICS_FIELDS,
      presets: GAME_GRAPHICS_PRESETS,
      footnote: "Changes take effect the next time you launch the game.",
    };
  }

  function openSaveConfig(s: Savegame) {
    if (!userDir) return;
    savesOpen = false;
    configEditor = {
      title: `Configure — ${s.name}`,
      path: `${userDir}/${s.folder}/careerSavegame.xml`,
      fields: SAVEGAME_FIELDS,
      presets: SAVEGAME_PRESETS,
      footnote: "Edits this savegame's settings. Back up the save first if unsure.",
    };
  }

  const libStats = $derived.by(() => {
    let totalSize = 0;
    let rated = 0;
    let ratingSum = 0;
    let tagged = 0;
    const byCat: Record<string, { count: number; size: number }> = {};
    for (const m of effectiveMods) {
      totalSize += m.size;
      (byCat[m.category] ??= { count: 0, size: 0 });
      byCat[m.category].count++;
      byCat[m.category].size += m.size;
      const r = cur(m.techName).rating;
      if (r > 0) {
        rated++;
        ratingSum += r;
      }
      if (tagsOf(m.techName).length > 0) tagged++;
    }
    const cats = Object.entries(byCat)
      .map(([name, v]) => ({ name, ...v }))
      .sort((a, b) => b.size - a.size);
    const largest = [...effectiveMods].sort((a, b) => b.size - a.size).slice(0, 8);
    const maxCatSize = cats.reduce((m, c) => Math.max(m, c.size), 1);
    return {
      totalSize,
      cats,
      maxCatSize,
      largest,
      rated,
      avgRating: rated ? ratingSum / rated : 0,
      tagged,
    };
  });
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

  // Every mod named in a non-info conflict, for the "In conflict" library filter.
  // Conflict.mods carries tech names (and sometimes titles), so we match on both.
  const conflictedSet = $derived.by(() => {
    const s = new Set<string>();
    for (const c of conflicts) {
      if (c.severity === "info") continue;
      for (const name of c.mods) s.add(name);
    }
    return s;
  });

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

  const saveRows = $derived(savegames.map((s) => ({ s, st: saveStats(s) })));

  async function loadSavegames() {
    try {
      savegames = await getSavegames();
    } catch (e) {
      errorMsg = String(e);
    }
  }

  function buildReport(): string {
    const L: string[] = [];
    L.push("# Silo diagnostics report");
    L.push(`Generated: ${new Date().toISOString()}`, "");
    L.push("## Environment");
    L.push(`- Mods folder: ${roots[0] ?? "not detected"}`);
    L.push(`- Game: ${gameInfo ? gameInfo.installDir : "not detected"}`, "");
    L.push("## Library");
    L.push(`- Mods: ${mods.length}  (organized ${organizedCount}, active ${activeSet.size})`);
    L.push(`- Total size: ${fmtSize(libStats.totalSize)}`);
    L.push(`- Maps: ${stats.maps}  ·  Script mods: ${stats.scripts}  ·  Tagged: ${libStats.tagged}`, "");
    L.push("### By category");
    for (const c of libStats.cats) L.push(`- ${c.name}: ${c.count}  (${fmtSize(c.size)})`);
    L.push("");
    const al = loadouts.find((l) => l.id === activeLoadoutId);
    L.push(`## Active set${al ? ` — loadout “${al.name}”` : ""} (${activeSet.size} mods)`, "");
    L.push(`## Conflicts (${criticalCount} critical, ${conflicts.length} total)`);
    for (const c of conflicts.filter((x) => x.severity !== "info")) {
      L.push(`- [${c.severity}] ${c.kind} “${c.name}”: ${c.mods.join(", ")}`);
    }
    L.push("");
    L.push(`## Health (${healthCount} issues)`);
    for (const d of health.missingDeps) {
      L.push(`- Missing dependency: ${d.mod.title ?? d.mod.techName} needs ${d.missing.join(", ")}`);
    }
    for (const m of health.ignored) L.push(`- Ignored (digit prefix): ${m.techName}`);
    for (const m of health.corrupt) L.push(`- Corrupt: ${m.techName} — ${m.error}`);
    return L.join("\n");
  }

  async function exportReport() {
    try {
      await saveTextFile("silo-report.md", buildReport());
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function backupSave(s: Savegame) {
    busy = `Backing up “${s.name}”…`;
    try {
      await backupSavegame(s.folder);
    } catch (e) {
      errorMsg = String(e);
    }
    busy = null;
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
      await applyActive(l.mods);
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

  // Apply an active set and surface any warnings the projection engine reports — e.g. it
  // refused to remove a file you'd swapped in at a mod's name. These come back in the
  // report, not as a thrown error, so a plain `await setActive` would drop them silently.
  async function applyActive(techNames: string[]) {
    const rep = await setActive(techNames);
    if (rep.errors.length) errorMsg = rep.errors.slice(0, 3).join("; ");
    return rep;
  }

  async function toggleActive(techName: string) {
    const next = new Set(activeSet);
    if (next.has(techName)) next.delete(techName);
    else next.add(techName);
    activeSet = next;
    try {
      await applyActive([...next]);
    } catch (e) {
      errorMsg = String(e);
    }
  }

  // Build the organize inputs for every loose (unorganized) zip currently in view.
  function looseInputs(): { inputs: ModInput[]; techNames: string[] } {
    const targets = effectiveMods.filter(isFileable);
    return {
      inputs: targets.map((m) => ({
        techName: m.techName,
        fileName: fileName(m.path),
        kind: m.kind,
        category: m.category,
        subcategory: m.subcategory,
      })),
      techNames: targets.map((m) => m.techName),
    };
  }

  // Apply a prepared set of organize inputs. `keepActive` re-projects them so their
  // loaded state is preserved (used by both the auto-filer and the manual button).
  async function applyLoose(inputs: ModInput[], techNames: string[], keepActive: boolean) {
    if (inputs.length === 0) return;
    busy = `Filing ${inputs.length} mod${inputs.length > 1 ? "s" : ""} into the library…`;
    try {
      const rep = await applyOrganize(inputs);
      if (rep.errors.length) {
        errorMsg = rep.errors.slice(0, 3).join("; ");
      } else if (keepActive) {
        const next = new Set([...activeSet, ...techNames]);
        activeSet = next;
        await applyActive([...next]);
      }
    } catch (e) {
      errorMsg = String(e);
    }
    busy = null;
    await runScan(false);
  }

  // Auto-filer (opt-in): file new mods and keep them active — no prompt, they chose it.
  async function autoFile() {
    const { inputs, techNames } = looseInputs();
    await applyLoose(inputs, techNames, true);
  }

  // ── Manual "Organize N" button → dry-run preview → confirm ───────────────────
  // A read-only preview of exactly what will move, shown before any file is touched.
  // Dismissible for good ("Don't preview next time"), so power users aren't nagged.
  let organizePreview = $state<{
    inputs: ModInput[];
    techNames: string[];
    plan: PlannedMove[];
    loading: boolean;
    applying: boolean;
  } | null>(null);

  const PREVIEW_SKIP_KEY = "silo.organizePreviewSkip";

  async function organizeNew() {
    const { inputs, techNames } = looseInputs();
    if (inputs.length === 0) return;
    // Honor a remembered "don't preview" choice.
    if (localStorage.getItem(PREVIEW_SKIP_KEY) === "true") {
      await applyLoose(inputs, techNames, true);
      return;
    }
    organizePreview = { inputs, techNames, plan: [], loading: true, applying: false };
    try {
      const plan = await planOrganize(inputs);
      if (organizePreview) organizePreview = { ...organizePreview, plan, loading: false };
    } catch (e) {
      errorMsg = String(e);
      organizePreview = null;
    }
  }

  async function confirmOrganize(skipNext: boolean) {
    if (!organizePreview) return;
    if (skipNext) localStorage.setItem(PREVIEW_SKIP_KEY, "true");
    const { inputs, techNames } = organizePreview;
    organizePreview = { ...organizePreview, applying: true };
    await applyLoose(inputs, techNames, true);
    organizePreview = null;
  }

  async function rebuildLibrary() {
    busy = "Rebuilding library (re-scanning every mod)…";
    errorMsg = null;
    try {
      await clearScanCache();
      await runScan(false);
    } catch (e) {
      errorMsg = String(e);
    }
    busy = null;
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

  async function setCategory(
    techName: string,
    category: string,
    subcategory: string | null = null,
  ) {
    overrideMap = {
      ...overrideMap,
      [techName]: { category, subcategory },
    };
    editing = null;
    try {
      await setOverride({ techName, category, subcategory });
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

  // GitHub repo links: techName -> {owner, repo}
  let repoMap = $state<Record<string, { owner: string; repo: string }>>({});
  async function loadRepos() {
    try {
      const rows = await getModRepos();
      repoMap = Object.fromEntries(rows.map((r) => [r.techName, { owner: r.owner, repo: r.repo }]));
    } catch (e) {
      errorMsg = String(e);
    }
  }

  let updatesOpen = $state(false);
  let updateChecking = $state(false);
  let updateResults = $state<UpdateRow[]>([]);
  const linkedCount = $derived(Object.keys(repoMap).length);

  async function checkAllUpdates() {
    updatesOpen = true;
    updateChecking = true;
    updateResults = [];
    const byTech = new Map(mods.map((m) => [m.techName, m]));
    const rows = new Map<string, UpdateRow>();

    // 1. One catalog request covers the whole library (GitHub + ModHub mods alike).
    try {
      const catalog = await catalogCheckUpdates(
        mods.map((m) => ({ techName: m.techName, version: m.version ?? undefined })),
      );
      for (const c of catalog) {
        const m = byTech.get(c.techName);
        if (!m) continue;
        rows.set(c.techName, {
          techName: c.techName,
          title: m.title ?? c.techName,
          path: m.path,
          current: m.version ?? undefined,
          latest: c.latest ?? undefined,
          hasUpdate: c.hasUpdate,
          assetUrl: c.downloadUrl,
          source: c.source ?? undefined,
        });
      }
      updateResults = [...rows.values()];
    } catch {
      // Catalog unreachable (e.g. endpoint not yet deployed) — fall back to GitHub only.
    }

    // 2. Linked GitHub repos are authoritative for the user's chosen source — check
    //    them directly and let them override the catalog row for that mod.
    for (const [techName, r] of Object.entries(repoMap)) {
      const m = byTech.get(techName);
      if (!m) continue;
      const row: UpdateRow =
        rows.get(techName) ?? { techName, title: m.title ?? techName, path: m.path };
      try {
        const info = await checkModUpdate(r.owner, r.repo, m.version ?? "0");
        row.current = info.current;
        row.latest = info.release.tag;
        row.hasUpdate = info.hasUpdate;
        row.assetUrl = info.release.assetUrl;
        row.source = "github";
        row.error = undefined;
      } catch (e) {
        if (!rows.has(techName)) row.error = String(e);
      }
      rows.set(techName, row);
      updateResults = [...rows.values()];
    }

    updateChecking = false;
  }

  async function installFromRow(row: UpdateRow) {
    if (!row.assetUrl) return;
    busy = `Installing ${row.title}…`;
    try {
      await downloadUpdate(row.path, row.assetUrl);
      updateResults = updateResults.map((r) =>
        r.techName === row.techName ? { ...r, hasUpdate: false, current: r.latest } : r,
      );
    } catch (e) {
      errorMsg = String(e);
    }
    busy = null;
    await runScan(false);
  }

  const availableUpdates = $derived(updateResults.filter((r) => r.hasUpdate));

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
    if (flaggedOnly) {
      list = list.filter((m) => cur(m.techName).broken);
    }
    if (conflictedOnly) {
      list = list.filter(
        (m) => conflictedSet.has(m.techName) || conflictedSet.has(m.title ?? ""),
      );
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

  type SortKey = "name" | "category" | "size" | "added" | "version" | "rating";
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
        case "rating":
          // My own star rating (0 = unrated sorts last on desc, which is the default use).
          r = cur(a.techName).rating - cur(b.techName).rating;
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
      await applyActive([...next]);
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
        await loadRepos();
        await loadLoadouts();
        await loadSavegames();
        gameInfo = await detectGame();
        userDir = await userDirPath();
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
      // A leftover snapshot means a bisection was interrupted — offer to restore.
      try {
        bisectRecovery = await bisectSnapshotGet();
      } catch {
        /* non-critical */
      }
    })();
    return () => unlisten?.();
  });

  async function restoreBisect() {
    if (!bisectRecovery) return;
    busy = "Restoring your mod set…";
    try {
      await applyActive(bisectRecovery);
      await bisectSnapshotClear();
      bisectRecovery = null;
      await runScan(false);
    } catch (e) {
      errorMsg = String(e);
    }
    busy = null;
  }
</script>

<div class="app" style="--topbar-h:{topbarH}px">
  <Topbar
    bind:topbarH
    {view}
    hasSavegames={savegames.length > 0}
    {savesOpen}
    {loadoutsOpen}
    activeLoadoutName={activeLoadoutId !== null
      ? loadouts.find((l) => l.id === activeLoadoutId)?.name ?? null
      : null}
    {mpOpen}
    showOrganize={unorganizedCount > 0 && !autoFileNew}
    {unorganizedCount}
    hasMods={mods.length > 0}
    {updateChecking}
    {scanning}
    {busy}
    hasGame={!!gameInfo}
    activeCount={activeSet.size}
    {settingsOpen}
    onSwitchView={switchView}
    onToggleSaves={() => (savesOpen = !savesOpen)}
    onToggleLoadouts={() => (loadoutsOpen = !loadoutsOpen)}
    onToggleMp={() => (mpOpen = !mpOpen)}
    onOrganize={organizeNew}
    onCheckUpdates={checkAllUpdates}
    onRescan={() => runScan()}
    onLaunch={launch}
    onToggleSettings={() => (settingsOpen = !settingsOpen)}
  />

  {#if bisectRecovery}
    <div class="recover-banner">
      <span>
        A guided diagnosis was interrupted — your active mods may be partially applied.
      </span>
      <div class="recover-actions">
        <button class="btn primary" onclick={restoreBisect} disabled={!!busy}>
          Restore my mod set
        </button>
        <button class="btn" onclick={() => (bisectRecovery = null)} disabled={!!busy}>
          Later
        </button>
      </div>
    </div>
  {/if}

  {#if loadoutsOpen}
    <LoadoutsPanel
      {loadouts}
      activeCount={activeSet.size}
      {activeLoadoutId}
      onApply={applyLoadout}
      onOverwrite={overwriteLoadout}
      onExport={exportLoadout}
      onRemove={removeLoadout}
      onSaveCurrent={saveCurrentLoadout}
      onImport={importLoadout}
      onClose={() => (loadoutsOpen = false)}
    />
  {/if}

  {#if savesOpen}
    <SavegamesPanel
      rows={saveRows}
      {busy}
      onConfigure={openSaveConfig}
      onBackup={backupSave}
      onLoadout={loadoutFromSave}
      onClose={() => (savesOpen = false)}
    />
  {/if}

  {#if conflictsOpen}
    <ConflictsPanel
      {conflicts}
      {criticalCount}
      activeCount={activeSet.size}
      onClose={() => (conflictsOpen = false)}
    />
  {/if}

  {#if settingsOpen}
    <SettingsPanel
      {theme}
      {roots}
      {gameInfo}
      {userDir}
      {autoFileNew}
      {organizedCount}
      {unorganizedCount}
      {busy}
      {scanning}
      onSetTheme={setTheme}
      onOpenGameGraphics={openGameGraphics}
      onExportReport={exportReport}
      onSetAutoFile={setAutoFile}
      onOrganize={() => {
        settingsOpen = false;
        organizeNew();
      }}
      onRebuild={rebuildLibrary}
      onRestoreVanilla={() => {
        settingsOpen = false;
        restoreVanilla();
      }}
      onClose={() => (settingsOpen = false)}
    />
  {/if}

  {#if settingsMod}
    <ModSettings
      modName={settingsMod.techName}
      title={settingsMod.title}
      onClose={() => (settingsMod = null)}
    />
  {/if}

  {#if configEditor}
    <ConfigEditor
      title={configEditor.title}
      path={configEditor.path}
      fields={configEditor.fields}
      presets={configEditor.presets}
      footnote={configEditor.footnote}
      onClose={() => (configEditor = null)}
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
      categories={CATEGORIES}
      isOverridden={overrideMap[dm.techName] != null}
      onSetCategory={(c, s) => setCategory(dm.techName, c, s)}
      onResetCategory={() => resetCategory(dm.techName)}
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
      repo={repoMap[dm.techName] ?? null}
      onRepoChange={(r) => {
        const next = { ...repoMap };
        if (r) next[dm.techName] = r;
        else delete next[dm.techName];
        repoMap = next;
      }}
      onInstalled={() => {
        detailMod = null;
        runScan(false);
      }}
    />
  {/if}

  {#if updatesOpen}
    <UpdatesPanel
      {updateResults}
      {availableUpdates}
      {updateChecking}
      {busy}
      onInstall={installFromRow}
      onClose={() => (updatesOpen = false)}
    />
  {/if}

  {#if statsOpen}
    <StatsPanel
      modCount={mods.length}
      {organizedCount}
      activeCount={activeSet.size}
      mapsCount={stats.maps}
      {libStats}
      onClose={() => (statsOpen = false)}
    />
  {/if}

  {#if healthOpen}
    <HealthPanel {health} {healthCount} onClose={() => (healthOpen = false)} />
  {/if}

  {#if logOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div class="backdrop" onclick={() => (logOpen = false)}></div>
    <div class="log-panel">
      <LogTriage
        onClose={() => (logOpen = false)}
        activeMods={[...activeSet]}
        activeMaps={mods.filter((m) => activeSet.has(m.techName) && m.isMap).map((m) => m.techName)}
      />
    </div>
  {/if}

  {#if bindingsOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div class="backdrop" onclick={() => (bindingsOpen = false)}></div>
    <div class="log-panel">
      <BindingsView onClose={() => (bindingsOpen = false)} />
    </div>
  {/if}

  {#if mpOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div class="backdrop" onclick={() => (mpOpen = false)}></div>
    <div class="log-panel">
      <MpSync active={activeModRefs} onClose={() => (mpOpen = false)} />
    </div>
  {/if}

  {#if bridgeOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div class="backdrop" onclick={() => (bridgeOpen = false)}></div>
    <div class="log-panel">
      <BridgeTool
        onGenerated={() => runScan(true)}
        onClose={() => (bridgeOpen = false)}
      />
    </div>
  {/if}

  {#if scanning}
    <div class="progress">
      <div class="bar" style="width: {pct}%"></div>
      <span class="progress-text tnum">{progress.done} / {progress.total}</span>
    </div>
  {/if}

  {#if organizePreview}
    <OrganizePreview
      plan={organizePreview.plan}
      loading={organizePreview.loading}
      applying={organizePreview.applying}
      onConfirm={confirmOrganize}
      onCancel={() => (organizePreview = null)}
    />
  {/if}

  {#if busy}
    <div class="busy">{busy}</div>
  {/if}

  {#if errorMsg}
    <div class="error">{errorMsg}</div>
  {/if}

  {#if view === "browse"}
    <!-- Own scroll region: the library scrolls via its inner VirtualList, but Browse
         is plain flow content, so it needs a flex-fill scroll container of its own. -->
    <div class="browse-scroll">
      <ModBrowser
        installed={libraryTechNames}
        onInstalled={() => runScan(true)}
        onNeedAuth={() => (settingsOpen = true)}
      />
    </div>
  {:else}
  <StatBar
    modCount={mods.length}
    mapsCount={stats.maps}
    scriptsCount={stats.scripts}
    uniqueCount={stats.unique}
    activeCount={activeSet.size}
    conflictCount={conflicts.length}
    {criticalCount}
    {healthCount}
    conflictedCount={conflictedSet.size}
    tookMs={result ? result.tookMs : null}
    bind:favoritesOnly
    bind:showHidden
    bind:flaggedOnly
    bind:conflictedOnly
    bind:query
    onOpenStats={() => (statsOpen = !statsOpen)}
    onOpenConflicts={() => (conflictsOpen = !conflictsOpen)}
    onOpenHealth={() => (healthOpen = !healthOpen)}
    onOpenLog={() => (logOpen = true)}
    onOpenBindings={() => (bindingsOpen = true)}
    onOpenBridge={() => (bridgeOpen = true)}
  />

  <div class="body">
    <CategoryRail
      items={effectiveMods}
      {selected}
      onSelect={(category, subcategory) => (selected = { category, subcategory })}
    />

    <main class="list">
      <LibraryToolbar
        {selected}
        {selectedTag}
        shownCount={filtered.length}
        allActive={allFilteredActive}
        activeInFilter={filteredActiveCount}
        disabled={!!busy || filtered.length === 0}
        bind:sortBy
        bind:sortDir
        onSelectAll={setActiveForFiltered}
        onClearTag={() => (selectedTag = null)}
      />

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
  {/if}

  {#if editing}
    <CategoryMenu
      categories={CATEGORIES}
      x={editing.x}
      y={editing.y}
      onSelect={(c) => setCategory(editing!.techName, c)}
      onReset={() => resetCategory(editing!.techName)}
      onClose={() => (editing = null)}
    />
  {/if}
</div>

<style>
  .app {
    display: flex;
    flex-direction: column;
    height: 100%;
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
  /* Log-triage panel owns its own padding (LogTriage.svelte), so no inner padding here. */
  .recover-banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    flex-wrap: wrap;
    margin: 0 0 10px;
    padding: 10px 14px;
    background: color-mix(in srgb, var(--warn) 14%, transparent);
    border: 1px solid color-mix(in srgb, var(--warn) 40%, transparent);
    border-radius: var(--radius-sm);
    color: var(--text);
    font-size: 0.88rem;
  }
  .recover-actions {
    display: flex;
    gap: 8px;
  }
  .log-panel {
    position: fixed;
    z-index: 50;
    top: 90px;
    left: 50%;
    transform: translateX(-50%);
    width: 620px;
    max-width: calc(100vw - 40px);
    max-height: 80vh;
    overflow-y: auto;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    box-shadow: var(--shadow-2);
    scrollbar-width: thin;
  }
  .body {
    flex: 1 1 auto;
    min-height: 0;
    display: flex;
    /* Reserve room for an open detail drawer so the list tucks left of it. */
    padding-right: var(--drawer-w, 0px);
  }
  .browse-scroll {
    flex: 1 1 auto;
    min-height: 0;
    overflow-y: auto;
    /* Reserve room for an open detail drawer (Browse content is centered, so this
       shifts it left of the drawer instead of hiding under it). */
    padding-right: var(--drawer-w, 0px);
  }
  .list {
    flex: 1 1 auto;
    min-width: 0;
    min-height: 0;
    display: flex;
    flex-direction: column;
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
