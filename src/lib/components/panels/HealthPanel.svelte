<script lang="ts">
  import type { ModEntry } from "../../types";

  let {
    health,
    healthCount,
    onClose,
  }: {
    health: {
      missingDeps: { mod: ModEntry; missing: string[] }[];
      corrupt: ModEntry[];
      ignored: ModEntry[];
    };
    healthCount: number;
    onClose: () => void;
  } = $props();
</script>

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class="backdrop" onclick={onClose}></div>
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

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    z-index: 40;
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
</style>
