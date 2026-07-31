<script lang="ts">
  type SortKey = "name" | "category" | "size" | "added" | "version" | "rating";

  let {
    selected,
    selectedTag,
    shownCount,
    allActive,
    activeInFilter,
    disabled,
    sortBy = $bindable(),
    sortDir = $bindable(),
    onSelectAll,
    onClearTag,
  }: {
    selected: { category: string | null; subcategory: string | null };
    selectedTag: string | null;
    shownCount: number;
    allActive: boolean;
    activeInFilter: number;
    disabled: boolean;
    sortBy: SortKey;
    sortDir: "asc" | "desc";
    onSelectAll: (checked: boolean) => void;
    onClearTag: () => void;
  } = $props();

  let selectAllEl = $state<HTMLInputElement>();
  $effect(() => {
    if (selectAllEl) {
      selectAllEl.indeterminate = activeInFilter > 0 && activeInFilter < shownCount;
    }
  });
</script>

<div class="crumb">
  <input
    type="checkbox"
    class="select-all"
    bind:this={selectAllEl}
    checked={allActive}
    {disabled}
    title="Activate / deactivate everything in this view"
    onchange={(e) => onSelectAll(e.currentTarget.checked)}
  />
  <span class="crumb-path">
    {#if selected.category}
      {selected.category}{selected.subcategory ? " › " + selected.subcategory : ""}
    {:else}
      All mods
    {/if}
  </span>
  {#if selectedTag}
    <button class="crumb-tag" onclick={onClearTag} title="Clear tag filter">
      #{selectedTag} ✕
    </button>
  {/if}
  <span class="crumb-count tnum">{shownCount} shown</span>

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
        <option value="rating">My rating</option>
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

<style>
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
</style>
