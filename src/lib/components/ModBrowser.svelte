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
    siloapiFacets,
    installRemoteMod,
    onInstallProgress,
    openExternal,
    ghStatus,
    nexusStatus,
    nexusModDescription,
    type Facets,
  } from "../api";
  import type {
    BrowseMod,
    SiloStats,
    CatalogModDetail,
    CategoryCount,
    ModSourceOption,
  } from "../types";
  import type { UnlistenFn } from "@tauri-apps/api/event";
  import ModCard from "./ModCard.svelte";
  import DescModal from "./DescModal.svelte";
  import BrowseDrawer from "./BrowseDrawer.svelte";
  import { parseNexusId } from "../browse";
  import { browseCacheKey, getBrowseView, setBrowseView } from "../browseCache";

  interface Props {
    /** Tech names already in the local library, to flag "in library". */
    installed: Set<string>;
    /** Called after a successful install so the parent can rescan. */
    onInstalled: (filename: string) => void;
    /** Route the user to the source-connect UI (Settings) from a source card. */
    onNeedAuth?: () => void;
    /** Pre-seed the catalog search (e.g. from a library row's "Find in Browse").
     *  Applied on mount; Browse remounts on every view switch, so this seeds each entry. */
    seed?: string | null;
  }
  let { installed, onInstalled, onNeedAuth, seed = null }: Props = $props();

  // Source connection state, for the interactive cards in the drawer.
  let gh = $state<{ connected: boolean; canWrite: boolean }>({
    connected: false,
    canWrite: false,
  });
  let nexusConnected = $state(false);
  async function refreshGh() {
    try {
      const s = await ghStatus();
      gh = { connected: !!s.user, canWrite: s.canWrite };
    } catch {
      gh = { connected: false, canWrite: false };
    }
    try {
      nexusConnected = !!(await nexusStatus()).user;
    } catch {
      nexusConnected = false;
    }
  }
  refreshGh();

  let query = $state("");
  // `category` is the API filter param, derived from the parent/child dropdowns below.
  let category = $state("");
  let parentCategory = $state("");
  let childCategory = $state("");
  let sort = $state<"popular" | "newest" | "name" | "downloads" | "rating">(
    "newest",
  );
  // Sort direction. The catalog API sorts are fixed-direction today (see silo-api), so on
  // Browse this arrow is disabled until the server honors it; the Library wires it client-side.
  let sortDir = $state<"desc" | "asc">("desc");
  let categories = $state<CategoryCount[]>([]);
  // Split the flat "Parent - Child" catalog categories into a parent → children tree so the
  // bar can show two dropdowns. A single-level category (e.g. "Gameplay") is a childless parent.
  const catTree = $derived.by(() => {
    const map = new Map<
      string,
      { name: string; count: number; children: { name: string; count: number; full: string }[] }
    >();
    for (const c of categories) {
      const i = c.category.indexOf(" - ");
      const parent = i >= 0 ? c.category.slice(0, i) : c.category;
      const child = i >= 0 ? c.category.slice(i + 3) : null;
      let node = map.get(parent);
      if (!node) {
        node = { name: parent, count: 0, children: [] };
        map.set(parent, node);
      }
      node.count += c.count;
      if (child) node.children.push({ name: child, count: c.count, full: c.category });
    }
    return [...map.values()].sort((a, b) => a.name.localeCompare(b.name));
  });
  const childList = $derived(catTree.find((n) => n.name === parentCategory)?.children ?? []);
  // Picking a parent resets the child; the API param becomes the parent (single-level works
  // now; a multi-child parent's "All" needs server parent-matching — silo-api#8).
  function setParent(p: string) {
    parentCategory = p;
    childCategory = "";
    category = p;
    load();
  }
  function setChild(full: string) {
    childCategory = full;
    category = full || parentCategory;
    load();
  }
  // Semantic facet filters (silo-api #4). activeTags are "namespace:value" strings, ANDed by
  // the server; availableBy is the period-correct year filter. facets drives a compact
  // dropdown row — one value per facet (picking a new value replaces the prior one).
  let facets = $state<Facets | null>(null);
  let activeTags = $state<string[]>([]);
  let availableBy = $state<number | null>(null);
  // Render facets in a sensible order; anything else the server adds falls in after.
  const FACET_ORDER = ["region", "era", "theme", "brand", "realism"];
  const FACET_LABEL: Record<string, string> = {
    theme: "Theme",
    brand: "Brand",
    region: "Region",
    realism: "Realism",
    era: "Era",
  };
  const facetGroups = $derived(
    facets
      ? Object.keys(facets.facets)
          .sort((a, b) => {
            const ia = FACET_ORDER.indexOf(a);
            const ib = FACET_ORDER.indexOf(b);
            return (ia < 0 ? 99 : ia) - (ib < 0 ? 99 : ib);
          })
          .filter((ns) => facets!.facets[ns].length > 0)
      : [],
  );
  const filterCount = $derived(
    activeTags.length + (availableBy != null ? 1 : 0) + (parentCategory ? 1 : 0),
  );

  function prettify(v: string): string {
    return v.replace(/-/g, " ").replace(/\b\w/g, (c) => c.toUpperCase());
  }
  // The currently-selected "ns:value" for a facet dropdown, or "" (= all) when none is picked.
  function selectedFor(ns: string): string {
    return activeTags.find((t) => t.split(":")[0] === ns) ?? "";
  }
  // Pick one value per facet; picking replaces any prior value for that namespace.
  function setFacet(ns: string, tag: string) {
    activeTags = [...activeTags.filter((t) => t.split(":")[0] !== ns), ...(tag ? [tag] : [])];
    load();
  }
  function clearFilters() {
    activeTags = [];
    availableBy = null;
    parentCategory = "";
    childCategory = "";
    category = "";
    load();
  }
  function setAvailableBy(v: string) {
    const n = parseInt(v, 10);
    availableBy = Number.isFinite(n) && n >= 1900 && n <= 2100 ? n : null;
    load();
  }
  let results = $state<BrowseMod[]>([]);
  let total = $state(0);
  let lastPageFull = $state(false);
  let loadingMore = $state(false);
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
  // Full-description modal (the drawer shows a clamped snippet). `source` records where
  // the body came from so we can label it honestly.
  let descModal = $state<{
    title: string;
    text: string;
    url: string | null;
    loading: boolean;
    source: "catalog" | "nexus" | "summary";
  } | null>(null);

  /** Open the description modal, filling it with the best body available: the ingested
   *  full text if the catalog has it, else a live Nexus fetch, else the short summary. */
  async function openDesc(d: CatalogModDetail) {
    const summary = d.description ?? "";
    if (d.descriptionFull) {
      descModal = { title: d.title, text: d.descriptionFull, url: d.pageUrl, loading: false, source: "catalog" };
      return;
    }
    const nx = d.sources.find((s) => s.source === "nexus" && parseNexusId(s.sourceUrl));
    if (!nx) {
      descModal = { title: d.title, text: summary, url: d.pageUrl, loading: false, source: "summary" };
      return;
    }
    // Show the summary immediately, then upgrade to the full Nexus body when it arrives.
    descModal = { title: d.title, text: summary, url: d.pageUrl, loading: true, source: "summary" };
    const id = parseNexusId(nx.sourceUrl)!;
    try {
      const full = await nexusModDescription(id);
      if (descModal && full && full.length > summary.length) {
        descModal.text = full;
        descModal.source = "nexus";
      }
    } catch {
      // Keep the summary; the deep-link still gets them the full page.
    } finally {
      if (descModal) descModal.loading = false;
    }
  }

  let debounce: ReturnType<typeof setTimeout> | null = null;
  let unlisten: UnlistenFn | null = null;

  async function openDetail(m: BrowseMod) {
    detail = null;
    detailLoading = true;
    // Re-read GitHub state so a connection made in Settings reflects in the cards.
    refreshGh();
    try {
      detail = await siloapiModDetail(m.id);
    } catch (e) {
      error = String(e);
    } finally {
      detailLoading = false;
    }
  }

  /** Click a source button: install it if we can, otherwise open its page. */
  async function useSource(m: { id: string }, s: ModSourceOption) {
    if (s.installable) await install(m, s.source);
    else await openExternal(s.sourceUrl);
  }

  function hasLocally(m: BrowseMod): boolean {
    return m.techName != null && installed.has(m.techName);
  }

  const PAGE = 60;
  // A server predating the `total` field reports 0. Don't render "of 0" or silently
  // hide Load-more — fall back to "the last page came back full, so there's probably
  // more", and just omit the total we don't have.
  const knowsTotal = $derived(total > 0);
  const hasMore = $derived(knowsTotal ? results.length < total : lastPageFull);

  /** Fetch the first page for the current filters, replacing what's shown. Served from the
   *  cross-mount view cache when we've seen this exact filter+sort combo before (no re-poll). */
  async function load() {
    error = null;
    const key = browseCacheKey({ category, tags: activeTags, availableBy, query, sort });
    const cached = getBrowseView(key);
    if (cached) {
      results = cached.results;
      total = cached.total;
      lastPageFull = cached.lastPageFull;
      loading = false;
      return;
    }
    loading = true;
    try {
      const page = await browseMods({
        query: query.trim() || undefined,
        category: category || undefined,
        sort,
        tags: activeTags,
        availableBy,
        limit: PAGE,
        offset: 0,
      });
      results = page.mods;
      total = page.total;
      lastPageFull = page.mods.length === PAGE;
      setBrowseView(key, { results, total, lastPageFull });
    } catch (e) {
      error = String(e);
      results = [];
      total = 0;
      lastPageFull = false;
    } finally {
      loading = false;
    }
  }

  /** Append the next page. Guards against double-firing and against the filters
   *  changing mid-flight, which would splice the wrong results onto the grid. */
  async function loadMore() {
    if (loadingMore || !hasMore) return;
    loadingMore = true;
    const forQuery = query;
    const forCategory = category;
    const forTags = activeTags.join("|");
    const forAvail = availableBy;
    const forSort = sort;
    try {
      const page = await browseMods({
        query: forQuery.trim() || undefined,
        category: forCategory || undefined,
        sort,
        tags: activeTags,
        availableBy,
        limit: PAGE,
        offset: results.length,
      });
      // filters moved on mid-flight → drop this page rather than splice wrong results
      if (
        forQuery !== query ||
        forCategory !== category ||
        forTags !== activeTags.join("|") ||
        forAvail !== availableBy ||
        forSort !== sort
      )
        return;
      results = [...results, ...page.mods];
      total = page.total;
      lastPageFull = page.mods.length === PAGE;
      // Grow the cached view so the next revisit restores everything we've paged in.
      setBrowseView(browseCacheKey({ category, tags: activeTags, availableBy, query, sort }), {
        results,
        total,
        lastPageFull,
      });
    } catch (e) {
      error = String(e);
    } finally {
      loadingMore = false;
    }
  }

  function onSearch() {
    if (debounce) clearTimeout(debounce);
    debounce = setTimeout(load, 300);
  }

  async function install(m: { id: string }, source?: string) {
    installing = m.id;
    error = null;
    installedNote = null;
    progress = { ...progress, [m.id]: { done: 0, total: null } };
    try {
      const filename = await installRemoteMod(m.id, source);
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
      // Sort alphabetically (the API returns them by mod count).
      categories = (await siloapiCategories()).sort((a, b) =>
        a.category.localeCompare(b.category, undefined, { sensitivity: "base" }),
      );
    } catch {
      // Older server without /categories — the filter just stays hidden.
    }
    try {
      facets = await siloapiFacets();
    } catch {
      // Older server without /facets — the filter panel just stays empty.
    }
    if (seed && seed.trim()) query = seed.trim();
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
  </div>

  <!-- Unified filter bar. Top line: category (parent · child) · search · sort · direction.
       Bottom line: the semantic facets · year · clear-all. -->
  <div class="filterbar">
    <div class="fb-row fb-top">
      {#if catTree.length > 0}
        <select
          class="fb-sel"
          class:on={parentCategory !== ""}
          value={parentCategory}
          onchange={(e) => setParent(e.currentTarget.value)}
          aria-label="Category"
        >
          <option value="">All categories</option>
          {#each catTree as n (n.name)}
            <option value={n.name}>{n.name} ({n.count.toLocaleString()})</option>
          {/each}
        </select>
        {#if childList.length > 0}
          <select
            class="fb-sel"
            class:on={childCategory !== ""}
            value={childCategory}
            onchange={(e) => setChild(e.currentTarget.value)}
            aria-label="Subcategory"
          >
            <option value="">All {parentCategory}</option>
            {#each childList as c (c.full)}
              <option value={c.full}>{c.name} ({c.count.toLocaleString()})</option>
            {/each}
          </select>
        {/if}
      {/if}
      <input
        class="fb-search"
        type="search"
        placeholder="Search the catalog…"
        bind:value={query}
        oninput={onSearch}
      />
      <select
        class="fb-sel fb-sort"
        bind:value={sort}
        onchange={() => load()}
        title="Sort order"
      >
        <option value="newest">Recently added / updated</option>
        <option value="popular">Popular</option>
        <option value="downloads">Most downloaded</option>
        <option value="rating">Top rated</option>
        <option value="name">Name (A–Z)</option>
      </select>
      <button
        class="fb-dir"
        disabled
        title="Sort direction — coming with a catalog update (silo-api)"
        aria-label="Sort direction (unavailable)"
      >
        {sortDir === "desc" ? "↓" : "↑"}
      </button>
    </div>

    {#if facetGroups.length > 0}
      <div class="fb-row fb-bottom">
        {#each facetGroups as ns (ns)}
          <select
            class="fb-sel"
            class:on={selectedFor(ns) !== ""}
            value={selectedFor(ns)}
            onchange={(e) => setFacet(ns, e.currentTarget.value)}
            aria-label="Filter by {FACET_LABEL[ns] ?? ns}"
          >
            <option value="">{FACET_LABEL[ns] ?? ns}: all</option>
            {#each facets!.facets[ns] as fv (fv.value)}
              <option value="{ns}:{fv.value}">{prettify(fv.value)} ({fv.count.toLocaleString()})</option>
            {/each}
          </select>
        {/each}
        <input
          class="fb-sel year-in"
          class:on={availableBy != null}
          type="number"
          min="1900"
          max="2100"
          placeholder="Year"
          title="Period-correct — only machines that existed by this year (dated mods only)."
          value={availableBy ?? ""}
          onchange={(e) => setAvailableBy(e.currentTarget.value)}
        />
        {#if filterCount > 0}
          <button class="clear-filters" onclick={clearFilters}>Clear all ✕</button>
        {/if}
      </div>
    {/if}
  </div>

  <div class="bh-status">
    {#if results.length > 0}
      <span class="showing tnum">
        Showing {results.length.toLocaleString()}{knowsTotal
          ? ` of ${total.toLocaleString()}`
          : ""}
      </span>
    {/if}
    {#if base}<span class="source-note">Catalog: {base}</span>{/if}
  </div>

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
        <ModCard
          {m}
          here={hasLocally(m)}
          installing={installing === m.id}
          progressEntry={progress[m.id]}
          onUseSource={(s) => useSource(m, s)}
          onOpenDetail={() => openDetail(m)}
        />
      {/each}
    </div>

    {#if hasMore}
      <div class="more">
        <button class="btn more-btn" disabled={loadingMore} onclick={loadMore}>
          {#if loadingMore}
            Loading…
          {:else if knowsTotal}
            Load {Math.min(PAGE, total - results.length)} more
          {:else}
            Load more
          {/if}
        </button>
      </div>
    {:else if knowsTotal && total > PAGE}
      <div class="more end tnum">That's all {total.toLocaleString()}.</div>
    {/if}
  {/if}

  {#if detailLoading || detail}
    <BrowseDrawer
      {detail}
      {detailLoading}
      installingId={installing}
      {gh}
      {nexusConnected}
      {installed}
      onClose={() => (detail = null)}
      onUseSource={(d, s) => useSource(d, s)}
      onOpenDesc={openDesc}
      {onNeedAuth}
    />
  {/if}

  {#if descModal}
    <DescModal modal={descModal} onClose={() => (descModal = null)} />
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
  /* ── Unified filter bar: top (category · search · sort · direction), bottom (facets · year · clear) ── */
  .filterbar {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin: 6px 0 10px;
  }
  .fb-row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 8px;
  }
  .fb-sel {
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
  .fb-sel.on {
    color: var(--primary);
    border-color: color-mix(in srgb, var(--primary) 45%, var(--border));
    background: color-mix(in srgb, var(--primary) 10%, transparent);
    font-weight: 600;
  }
  .fb-search {
    flex: 1;
    min-width: 200px;
    padding: 9px 12px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface-raised);
    color: var(--text);
    font: inherit;
    font-size: 0.85rem;
  }
  .fb-search:focus {
    outline: 2px solid color-mix(in srgb, var(--primary) 55%, transparent);
    outline-offset: 1px;
  }
  /* Sort is ordering, not filtering — contrasting tint (matches the app's primary accent). */
  .fb-sort {
    border-color: color-mix(in srgb, var(--primary) 55%, var(--border));
    background: color-mix(in srgb, var(--primary) 14%, var(--surface-raised));
    color: var(--primary);
    font-weight: 700;
  }
  .fb-sort:hover {
    border-color: var(--primary);
  }
  .fb-dir {
    width: 34px;
    padding: 9px 0;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface-raised);
    color: var(--text);
    font: inherit;
    font-size: 0.95rem;
    line-height: 1;
    cursor: pointer;
  }
  .fb-dir:disabled {
    opacity: 0.45;
    cursor: default;
  }
  .year-in {
    width: 110px;
    max-width: 110px;
    cursor: text;
  }
  .year-in::placeholder {
    color: var(--text-muted);
  }
  .clear-filters {
    margin-left: auto;
    border: none;
    background: transparent;
    color: var(--text-muted);
    font: inherit;
    font-size: 0.8rem;
    font-weight: 600;
    cursor: pointer;
    padding: 6px;
  }
  .clear-filters:hover {
    color: var(--primary);
    text-decoration: underline;
  }
  .bh-status {
    display: flex;
    align-items: baseline;
    gap: 12px;
    margin: 4px 0 12px;
    flex-wrap: wrap;
  }
  .showing {
    font-size: 0.78rem;
    color: var(--text-muted);
  }
  .source-note {
    color: var(--text-muted);
    font-size: 0.75rem;
    margin-left: auto;
    opacity: 0.8;
  }
  .more {
    display: flex;
    justify-content: center;
    padding: 24px 0 8px;
  }
  .more.end {
    color: var(--text-muted);
    font-size: 0.78rem;
  }
  .more-btn {
    padding: 9px 22px;
    cursor: pointer;
  }
  .more-btn:disabled {
    opacity: 0.55;
    cursor: default;
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
  .btn:disabled {
    opacity: 0.55;
    cursor: default;
  }
</style>
