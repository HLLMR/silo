<script lang="ts" generics="T">
  import type { Snippet } from "svelte";

  interface Props {
    items: T[];
    rowHeight?: number;
    overscan?: number;
    row: Snippet<[T, number]>;
  }

  let { items, rowHeight = 76, overscan = 8, row }: Props = $props();

  let viewport = $state<HTMLDivElement>();
  let scrollTop = $state(0);
  let viewportH = $state(600);

  const total = $derived(items.length);
  const start = $derived(
    Math.max(0, Math.floor(scrollTop / rowHeight) - overscan),
  );
  const visible = $derived(Math.ceil(viewportH / rowHeight) + overscan * 2);
  const end = $derived(Math.min(total, start + visible));
  const slice = $derived(items.slice(start, end));

  function onScroll() {
    if (viewport) scrollTop = viewport.scrollTop;
  }
</script>

<div
  class="viewport"
  bind:this={viewport}
  bind:clientHeight={viewportH}
  onscroll={onScroll}
>
  <div class="sizer" style="height: {total * rowHeight}px">
    <div class="window" style="transform: translateY({start * rowHeight}px)">
      {#each slice as item, i (start + i)}
        <div class="cell" style="height: {rowHeight}px">
          {@render row(item, start + i)}
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .viewport {
    height: 100%;
    overflow-y: auto;
    overflow-x: hidden;
    scrollbar-width: thin;
  }
  .sizer {
    position: relative;
    width: 100%;
  }
  .window {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    will-change: transform;
  }
  .cell {
    box-sizing: border-box;
  }
</style>
