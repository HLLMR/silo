<script lang="ts">
  import type { ModEntry } from "../types";

  let { mod }: { mod: ModEntry } = $props();

  const sizeLabel = $derived(formatSize(mod.size));
  function formatSize(bytes: number): string {
    if (bytes >= 1024 * 1024 * 1024)
      return (bytes / 1024 ** 3).toFixed(1) + " GB";
    if (bytes >= 1024 * 1024) return (bytes / 1024 ** 2).toFixed(0) + " MB";
    if (bytes >= 1024) return (bytes / 1024).toFixed(0) + " KB";
    return bytes + " B";
  }

  // Initial for the placeholder tile (icon decode comes in a later slice).
  const initial = $derived(
    (mod.title ?? mod.techName).trim().charAt(0).toUpperCase() || "?",
  );
</script>

<div class="row" class:has-error={!!mod.error}>
  <div class="tile" class:map={mod.isMap}>{initial}</div>

  <div class="main">
    <div class="titleline">
      <span class="title">{mod.title ?? mod.techName}</span>
      {#if mod.version}<span class="ver tnum">v{mod.version}</span>{/if}
    </div>
    <div class="sub">
      <span class="author">{mod.author ?? "Unknown author"}</span>
      <span class="dot">·</span>
      <span class="tech">{mod.techName}</span>
    </div>
  </div>

  <div class="badges">
    {#if mod.isMap}<span class="badge map">Map</span>{/if}
    {#if mod.mpSupported}<span class="badge mp">MP</span>{/if}
    {#if mod.scriptCount > 0}
      <span class="badge script" title="Injects {mod.scriptCount} Lua script(s)">
        {mod.scriptCount} script{mod.scriptCount > 1 ? "s" : ""}
      </span>
    {/if}
    {#if mod.uniqueType}
      <span class="badge unique" title="uniqueType: {mod.uniqueType}">unique</span>
    {/if}
    {#if mod.dependencies.length > 0}
      <span class="badge dep" title={mod.dependencies.join(", ")}>
        {mod.dependencies.length} dep{mod.dependencies.length > 1 ? "s" : ""}
      </span>
    {/if}
    {#if mod.ignoredDigitPrefix}
      <span class="badge warn" title="Name starts with a digit — the game ignores this mod">
        ignored
      </span>
    {/if}
    {#if mod.error}
      <span class="badge err" title={mod.error}>error</span>
    {/if}
  </div>

  <div class="size tnum">{sizeLabel}</div>
</div>

<style>
  .row {
    display: flex;
    align-items: center;
    gap: 14px;
    height: 100%;
    padding: 0 16px;
    border-bottom: 1px solid var(--border);
  }
  .row:hover {
    background: color-mix(in srgb, var(--primary) 6%, transparent);
  }
  .tile {
    flex: 0 0 auto;
    width: 46px;
    height: 46px;
    border-radius: var(--radius-sm);
    display: grid;
    place-items: center;
    font-family: var(--font-display);
    font-size: 20px;
    font-weight: 600;
    color: var(--on-primary);
    background: linear-gradient(135deg, var(--green-500), var(--green-700));
    box-shadow: var(--shadow-1);
  }
  .tile.map {
    background: linear-gradient(135deg, var(--soil-500), var(--soil-700));
  }
  .main {
    flex: 1 1 auto;
    min-width: 0;
  }
  .titleline {
    display: flex;
    align-items: baseline;
    gap: 8px;
  }
  .title {
    font-weight: 600;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .ver {
    color: var(--text-muted);
    font-size: 12px;
  }
  .sub {
    color: var(--text-muted);
    font-size: 12.5px;
    display: flex;
    gap: 6px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .tech {
    opacity: 0.7;
  }
  .badges {
    flex: 0 0 auto;
    display: flex;
    gap: 6px;
    align-items: center;
  }
  .badge {
    font-size: 11px;
    font-weight: 600;
    padding: 2px 8px;
    border-radius: 999px;
    border: 1px solid var(--border);
    color: var(--text-muted);
    white-space: nowrap;
  }
  .badge.map {
    color: var(--soil-500);
    border-color: color-mix(in srgb, var(--soil-500) 40%, var(--border));
  }
  .badge.mp {
    color: var(--info);
    border-color: color-mix(in srgb, var(--info) 40%, var(--border));
  }
  .badge.unique,
  .badge.warn {
    color: var(--warn);
    border-color: color-mix(in srgb, var(--warn) 45%, var(--border));
  }
  .badge.err {
    color: var(--danger);
    border-color: color-mix(in srgb, var(--danger) 45%, var(--border));
  }
  .badge.dep {
    color: var(--accent);
    border-color: color-mix(in srgb, var(--accent) 45%, var(--border));
  }
  .size {
    flex: 0 0 auto;
    width: 64px;
    text-align: right;
    color: var(--text-muted);
    font-size: 12px;
  }
  .has-error {
    background: color-mix(in srgb, var(--danger) 5%, transparent);
  }
</style>
