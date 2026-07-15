<script lang="ts">
  import type { ModEntry } from "../types";

  interface Selected {
    category: string | null;
    subcategory: string | null;
  }
  interface Props {
    items: ModEntry[];
    selected: Selected;
    onSelect: (category: string | null, subcategory: string | null) => void;
  }
  let { items, selected, onSelect }: Props = $props();

  let expanded = $state<Set<string>>(new Set());

  // category -> { count, subs: [ [sub, count], ... ] }, sorted by count desc.
  const tree = $derived.by(() => {
    const m = new Map<string, { count: number; subs: Map<string, number> }>();
    for (const mod of items) {
      let e = m.get(mod.category);
      if (!e) {
        e = { count: 0, subs: new Map() };
        m.set(mod.category, e);
      }
      e.count++;
      if (mod.subcategory) {
        e.subs.set(mod.subcategory, (e.subs.get(mod.subcategory) ?? 0) + 1);
      }
    }
    return [...m.entries()]
      .sort((a, b) => b[1].count - a[1].count || a[0].localeCompare(b[0]))
      .map(([cat, info]) => ({
        cat,
        count: info.count,
        subs: [...info.subs.entries()].sort((a, b) => b[1] - a[1] || a[0].localeCompare(b[0])),
      }));
  });

  function clickCategory(cat: string) {
    onSelect(cat, null);
    const next = new Set(expanded);
    if (next.has(cat)) next.delete(cat);
    else next.add(cat);
    expanded = next;
  }
</script>

<nav class="rail">
  <button
    class="row all"
    class:active={!selected.category}
    onclick={() => onSelect(null, null)}
  >
    <span class="label">All mods</span>
    <span class="count tnum">{items.length}</span>
  </button>

  {#each tree as node (node.cat)}
    <button
      class="row cat"
      class:active={selected.category === node.cat && !selected.subcategory}
      onclick={() => clickCategory(node.cat)}
    >
      <span class="caret" class:open={expanded.has(node.cat)}>
        {node.subs.length ? "›" : ""}
      </span>
      <span class="label">{node.cat}</span>
      <span class="count tnum">{node.count}</span>
    </button>

    {#if expanded.has(node.cat)}
      {#each node.subs as [sub, n] (sub)}
        <button
          class="row sub"
          class:active={selected.category === node.cat && selected.subcategory === sub}
          onclick={() => onSelect(node.cat, sub)}
        >
          <span class="label">{sub}</span>
          <span class="count tnum">{n}</span>
        </button>
      {/each}
    {/if}
  {/each}
</nav>

<style>
  .rail {
    width: 232px;
    flex: 0 0 auto;
    overflow-y: auto;
    border-right: 1px solid var(--border);
    background: var(--surface);
    padding: 8px;
    scrollbar-width: thin;
  }
  .row {
    display: flex;
    align-items: center;
    gap: 6px;
    width: 100%;
    text-align: left;
    border: none;
    background: transparent;
    color: var(--text-muted);
    padding: 7px 10px;
    border-radius: var(--radius-sm);
    font-size: 13px;
    line-height: 1.2;
  }
  .row:hover {
    background: color-mix(in srgb, var(--primary) 8%, transparent);
    color: var(--text);
  }
  .row.active {
    background: color-mix(in srgb, var(--primary) 16%, transparent);
    color: var(--primary);
    font-weight: 600;
  }
  .all {
    font-weight: 600;
    color: var(--text);
    margin-bottom: 4px;
  }
  .label {
    flex: 1 1 auto;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .count {
    flex: 0 0 auto;
    font-size: 11.5px;
    opacity: 0.8;
  }
  .caret {
    flex: 0 0 auto;
    width: 10px;
    font-size: 11px;
    transition: transform 0.12s ease;
    color: var(--text-muted);
  }
  .caret.open {
    transform: rotate(90deg);
  }
  .sub {
    padding-left: 30px;
    font-size: 12.5px;
  }
</style>
