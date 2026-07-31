<script lang="ts">
  import type { BrowseMod, ModSourceOption } from "../types";
  import { label, shortLabel, gatedReason, fmtCount, fmtMB } from "../browse";

  let {
    m,
    here,
    installing,
    progressEntry,
    onUseSource,
    onOpenDetail,
  }: {
    m: BrowseMod;
    here: boolean;
    installing: boolean;
    progressEntry: { done: number; total: number | null } | undefined;
    onUseSource: (s: ModSourceOption) => void;
    onOpenDetail: () => void;
  } = $props();

  const pct = $derived.by(() => {
    const p = progressEntry;
    if (!p || !p.total) return null;
    return Math.min(100, Math.round((p.done / p.total) * 100));
  });
</script>

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
    {#if m.rating != null || m.downloads != null}
      <div class="stats">
        {#if m.rating != null}
          <span class="stat" title="Rating">
            ⭐ {m.rating.toFixed(1)}{#if m.ratingCount}<span class="stat-sub"
                >&nbsp;({fmtCount(m.ratingCount)})</span
              >{/if}
          </span>
        {/if}
        {#if m.downloads != null}
          <span class="stat" title="Downloads">⬇ {fmtCount(m.downloads)}</span>
        {/if}
      </div>
    {/if}
    {#if m.category}<div class="chip">{m.category}</div>{/if}
    {#if installing}
      {@const p = progressEntry}
      <div class="dl">
        <div class="dl-bar">
          <div
            class="dl-fill"
            class:indet={pct === null}
            style={pct !== null ? `width:${pct}%` : ""}
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
    <!-- One button per source this mod actually lives on, each with that
         source's own version — they drift, and that's worth seeing. -->
    <div class="srcbar">
      {#each m.sources as s (s.source)}
        <button
          class="srcbtn"
          class:can-install={s.installable}
          disabled={here || installing}
          title={here
            ? "Already in your library"
            : s.installable
              ? `Install from ${label(s.source)}`
              : gatedReason(s.source)}
          onclick={() => onUseSource(s)}
        >
          <span class="srcbtn-name">{shortLabel(s.source)}</span>
          {#if s.version}<span class="srcbtn-ver tnum">{s.version}</span>{/if}
          <span class="srcbtn-icon">{s.installable ? "⬇" : "↗"}</span>
        </button>
      {:else}
        <span class="srcbar-none">No sources</span>
      {/each}
    </div>
    <div class="card-actions">
      {#if here}<span class="card-owned">In library</span>{/if}
      <button class="btn ghost" title="Show details and sources" onclick={onOpenDetail}>
        Details
      </button>
    </div>
  </div>
</div>

<style>
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
  .stats {
    display: flex;
    gap: 10px;
    align-items: center;
    font-size: 0.8rem;
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
  }
  .stat {
    display: inline-flex;
    align-items: center;
    white-space: nowrap;
  }
  .stat-sub {
    color: var(--text-faint, var(--text-muted));
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
  /* One button per source, each showing that source's own version. */
  .srcbar {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-top: auto;
    padding-top: 8px;
  }
  .srcbar-none {
    font-size: 0.75rem;
    color: var(--text-muted);
  }
  .srcbtn {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    padding: 5px 8px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
    color: var(--text-muted);
    font: inherit;
    font-size: 0.75rem;
    cursor: pointer;
    transition: background 0.15s, border-color 0.15s, color 0.15s;
  }
  .srcbtn:hover:not(:disabled) {
    background: var(--bg);
    color: var(--text);
  }
  /* Directly installable reads as the primary action; the rest are link-outs. */
  .srcbtn.can-install {
    border-color: var(--primary);
    color: var(--primary);
    font-weight: 600;
  }
  .srcbtn.can-install:hover:not(:disabled) {
    background: var(--primary);
    color: var(--on-primary);
  }
  .srcbtn:disabled {
    opacity: 0.5;
    cursor: default;
  }
  .srcbtn-name {
    font-weight: 600;
  }
  .srcbtn-ver {
    opacity: 0.85;
  }
  .srcbtn-icon {
    opacity: 0.7;
    font-size: 0.7rem;
  }
  .card-owned {
    font-size: 0.75rem;
    color: var(--primary);
    font-weight: 600;
    align-self: center;
  }
  .card-actions {
    display: flex;
    gap: 8px;
    justify-content: space-between;
    align-items: center;
    padding-top: 8px;
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
  .btn.ghost {
    flex: 0 0 auto;
  }
  .btn:disabled {
    opacity: 0.55;
    cursor: default;
  }
</style>
