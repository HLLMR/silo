<script lang="ts">
  let {
    modCount,
    mapsCount,
    scriptsCount,
    uniqueCount,
    activeCount,
    conflictCount,
    criticalCount,
    healthCount,
    conflictedCount,
    updatesCount,
    tookMs,
    favoritesOnly = $bindable(),
    showHidden = $bindable(),
    flaggedOnly = $bindable(),
    conflictedOnly = $bindable(),
    hasSettingsOnly = $bindable(),
    needsUpdateOnly = $bindable(),
    statFilter = $bindable(),
    query = $bindable(),
    onOpenStats,
    onOpenConflicts,
    onOpenHealth,
    onOpenLog,
    onOpenBindings,
    onOpenBridge,
  }: {
    modCount: number;
    mapsCount: number;
    scriptsCount: number;
    uniqueCount: number;
    activeCount: number;
    conflictCount: number;
    criticalCount: number;
    healthCount: number;
    conflictedCount: number;
    updatesCount: number;
    tookMs: number | null;
    favoritesOnly: boolean;
    showHidden: boolean;
    flaggedOnly: boolean;
    conflictedOnly: boolean;
    hasSettingsOnly: boolean;
    needsUpdateOnly: boolean;
    statFilter: "" | "maps" | "scripts" | "unique" | "active";
    query: string;
    onOpenStats: () => void;
    onOpenConflicts: () => void;
    onOpenHealth: () => void;
    onOpenLog: () => void;
    onOpenBindings: () => void;
    onOpenBridge: () => void;
  } = $props();
</script>

<div class="statbar">
  <button class="stat statbtn" title="Library statistics" onclick={onOpenStats}>
    <span class="stat-num tnum">{modCount}</span>
    <span class="stat-label">mods</span>
  </button>
  <button
    class="stat statbtn"
    class:sel={statFilter === "maps"}
    title="Filter to maps only"
    onclick={() => (statFilter = statFilter === "maps" ? "" : "maps")}
  >
    <span class="stat-num tnum">{mapsCount}</span>
    <span class="stat-label">maps</span>
  </button>
  <button
    class="stat statbtn"
    class:sel={statFilter === "scripts"}
    title="Filter to script mods only"
    onclick={() => (statFilter = statFilter === "scripts" ? "" : "scripts")}
  >
    <span class="stat-num tnum">{scriptsCount}</span>
    <span class="stat-label">script mods</span>
  </button>
  <button
    class="stat statbtn"
    class:sel={statFilter === "unique"}
    title="Filter to mods that declare a uniqueType"
    onclick={() => (statFilter = statFilter === "unique" ? "" : "unique")}
  >
    <span class="stat-num tnum">{uniqueCount}</span>
    <span class="stat-label">uniqueType</span>
  </button>
  <button
    class="stat statbtn"
    class:sel={statFilter === "active"}
    title="Filter to the active set only"
    onclick={() => (statFilter = statFilter === "active" ? "" : "active")}
  >
    <span class="stat-num tnum">{activeCount}</span>
    <span class="stat-label">active</span>
  </button>
  <button
    class="stat statbtn"
    class:flag={conflictCount > 0}
    class:crit={criticalCount > 0}
    title="Conflicts within the active set"
    onclick={onOpenConflicts}
  >
    <span class="stat-num tnum">{conflictCount}</span>
    <span class="stat-label">conflict{conflictCount === 1 ? "" : "s"}</span>
  </button>
  <button
    class="stat statbtn"
    class:flag={healthCount > 0}
    title="Library health: missing dependencies, corrupt mods, ignored names"
    onclick={onOpenHealth}
  >
    <span class="stat-num tnum">{healthCount}</span>
    <span class="stat-label">need attention</span>
  </button>
  <button
    class="stat statbtn"
    title="Crash & log triage: did the last run crash, and which mod is at fault?"
    onclick={onOpenLog}
  >
    <span class="stat-num">◆</span>
    <span class="stat-label">diagnose</span>
  </button>
  <button
    class="stat statbtn"
    title="The full control-binding map — every action and key, grouped by device"
    onclick={onOpenBindings}
  >
    <span class="stat-num">⌨</span>
    <span class="stat-label">bindings</span>
  </button>
  <button
    class="stat statbtn"
    title="Filltype bridge: make a stubborn map filltype work with your equipment"
    onclick={onOpenBridge}
  >
    <span class="stat-num">⛓</span>
    <span class="stat-label">bridge</span>
  </button>
  {#if tookMs !== null}
    <div class="took tnum" title="Scan wall-clock time">
      scanned in {tookMs} ms
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
  <button
    class="toggle"
    class:on={flaggedOnly}
    title="Show only mods you've flagged as broken"
    onclick={() => (flaggedOnly = !flaggedOnly)}
  >
    ⚑ Flagged
  </button>
  <button
    class="toggle"
    class:on={conflictedOnly}
    title="Show only mods involved in a conflict"
    onclick={() => (conflictedOnly = !conflictedOnly)}
    disabled={conflictedCount === 0}
  >
    ⚠ In conflict{conflictedCount > 0 ? ` (${conflictedCount})` : ""}
  </button>
  <button
    class="toggle"
    class:on={hasSettingsOnly}
    title="Show only mods that expose in-game settings"
    onclick={() => (hasSettingsOnly = !hasSettingsOnly)}
  >
    ⚙ Has settings
  </button>
  <button
    class="toggle upd-toggle"
    class:on={needsUpdateOnly}
    title={updatesCount === 0
      ? "Run ⟳ Updates first to check for available updates"
      : "Show only mods with an available update"}
    onclick={() => (needsUpdateOnly = !needsUpdateOnly)}
    disabled={updatesCount === 0}
  >
    ⬆ Needs update{updatesCount > 0 ? ` (${updatesCount})` : ""}
  </button>

  <input
    class="search"
    type="search"
    placeholder="Filter by title, author, or tech name…"
    bind:value={query}
  />
</div>

<style>
  .statbar {
    display: flex;
    align-items: center;
    gap: 12px 20px;
    flex-wrap: wrap;
    /* Reserve room on the right for an open detail drawer (the subheader tucks left
       of it instead of being clipped). */
    padding: 12px calc(20px + var(--drawer-w, 0px)) 12px 20px;
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
  /* A stat acting as an active filter — highlight the number + underline the label. */
  .statbtn.sel .stat-num {
    color: var(--primary);
  }
  .statbtn.sel .stat-label {
    color: var(--primary);
    text-decoration: underline;
    text-underline-offset: 2px;
  }
  .statbtn.flag .stat-num {
    color: var(--warn);
  }
  .statbtn.crit .stat-num {
    color: var(--danger);
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
  .toggle:disabled {
    opacity: 0.45;
    cursor: default;
  }
  /* Updates are actionable — tint the toggle gold like the topbar update button. */
  .upd-toggle.on {
    background: color-mix(in srgb, var(--gold-700) 15%, transparent);
    border-color: color-mix(in srgb, var(--gold-700) 45%, var(--border));
    color: var(--gold-700);
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
</style>
