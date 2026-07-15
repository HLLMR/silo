<script lang="ts">
  import type { ModEntry, CurationRow, Conflict, UpdateInfo } from "../types";
  import {
    getModIcon,
    setCuration,
    setTags,
    revealInFolder,
    setModRepo,
    checkModUpdate,
    downloadUpdate,
    openExternal,
  } from "../api";

  type Flag = "favorite" | "hidden" | "broken";
  interface Props {
    mod: ModEntry;
    curation: CurationRow;
    tags: string[];
    active: boolean;
    organized: boolean;
    hasSettings: boolean;
    libraryTechNames: Set<string>;
    conflicts: Conflict[];
    onClose: () => void;
    onToggle: (flag: Flag) => void;
    onToggleActive: () => void;
    onOpenSettings: () => void;
    onCurationChange: (row: CurationRow) => void;
    onTagsChange: (tags: string[]) => void;
    onFilterTag: (tag: string) => void;
    repo: { owner: string; repo: string } | null;
    onRepoChange: (r: { owner: string; repo: string } | null) => void;
    onInstalled: () => void;
  }
  let {
    mod,
    curation,
    tags,
    active,
    organized,
    hasSettings,
    libraryTechNames,
    conflicts,
    onClose,
    onToggle,
    onToggleActive,
    onOpenSettings,
    onCurationChange,
    onTagsChange,
    onFilterTag,
    repo,
    onRepoChange,
    onInstalled,
  }: Props = $props();

  let installing = $state(false);
  async function installUpdate() {
    if (!update?.release.assetUrl) return;
    if (!confirm(`Download and install ${update.release.tag}? The current file is backed up to .bak.`))
      return;
    installing = true;
    ghError = null;
    try {
      await downloadUpdate(mod.path, update.release.assetUrl);
      onInstalled();
    } catch (e) {
      ghError = String(e);
    }
    installing = false;
  }

  let repoInput = $state("");
  let update = $state<UpdateInfo | null>(null);
  let checking = $state(false);
  let ghError = $state<string | null>(null);
  $effect(() => {
    repoInput = repo ? `${repo.owner}/${repo.repo}` : "";
    update = null;
    ghError = null;
  });

  async function linkRepo() {
    const parts = repoInput.trim().replace(/^https?:\/\/github\.com\//i, "").split("/");
    const owner = parts[0]?.trim() ?? "";
    const r = parts[1]?.trim() ?? "";
    try {
      await setModRepo(mod.techName, owner, r);
      onRepoChange(owner && r ? { owner, repo: r } : null);
    } catch (e) {
      ghError = String(e);
    }
  }

  async function checkUpdate() {
    if (!repo) return;
    checking = true;
    ghError = null;
    update = null;
    try {
      update = await checkModUpdate(repo.owner, repo.repo, mod.version ?? "0");
    } catch (e) {
      ghError = String(e);
    }
    checking = false;
  }

  let icon = $state<string | null>(null);
  $effect(() => {
    const m = mod;
    icon = null;
    if (m.iconFilename) {
      getModIcon(m.path, m.kind, m.iconFilename).then((u) => {
        if (u && mod.path === m.path) icon = u;
      });
    }
  });

  let noteDraft = $state("");
  $effect(() => {
    noteDraft = curation.note ?? "";
  });

  let newTag = $state("");

  function persistCuration(patch: Partial<CurationRow>) {
    const next: CurationRow = { ...curation, ...patch };
    onCurationChange(next);
    setCuration(next).catch(() => {});
  }
  function setRating(r: number) {
    persistCuration({ rating: curation.rating === r ? 0 : r });
  }
  function saveNote() {
    if ((curation.note ?? "") !== noteDraft) persistCuration({ note: noteDraft });
  }
  function addTag() {
    const t = newTag.trim();
    if (!t || tags.includes(t)) {
      newTag = "";
      return;
    }
    const next = [...tags, t];
    newTag = "";
    onTagsChange(next);
    setTags(mod.techName, next).catch(() => {});
  }
  function removeTag(t: string) {
    const next = tags.filter((x) => x !== t);
    onTagsChange(next);
    setTags(mod.techName, next).catch(() => {});
  }

  const label = $derived(mod.title ?? mod.techName);
  const myConflicts = $derived(conflicts.filter((c) => c.mods.includes(label)));
  const sizeLabel = $derived(formatSize(mod.size));
  function formatSize(b: number): string {
    if (b >= 1024 ** 3) return (b / 1024 ** 3).toFixed(1) + " GB";
    if (b >= 1024 ** 2) return (b / 1024 ** 2).toFixed(0) + " MB";
    if (b >= 1024) return (b / 1024).toFixed(0) + " KB";
    return b + " B";
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class="backdrop" onclick={onClose}></div>
<aside class="drawer">
  <div class="d-top">
    {#if icon}
      <img class="d-icon" src={icon} alt="" />
    {:else}
      <div class="d-icon ph">{label.charAt(0).toUpperCase()}</div>
    {/if}
    <div class="d-head">
      <div class="d-title">{label}</div>
      <div class="d-sub">
        {mod.author ?? "Unknown author"}{mod.version ? ` · v${mod.version}` : ""}
      </div>
      <div class="d-tech tnum">{mod.techName}</div>
    </div>
    <button class="d-x" onclick={onClose} aria-label="Close">✕</button>
  </div>

  <div class="d-actions">
    <button class="d-act" class:on={active} onclick={onToggleActive}>
      {active ? "● Active" : "○ Parked"}
    </button>
    <button class="d-act" class:on={curation.favorite} onclick={() => onToggle("favorite")}>
      {curation.favorite ? "★" : "☆"} Favorite
    </button>
    <button class="d-act" class:on={curation.broken} onclick={() => onToggle("broken")}>
      ⚠ Broken
    </button>
    <button class="d-act" class:on={curation.hidden} onclick={() => onToggle("hidden")}>
      ⊘ Hidden
    </button>
    {#if hasSettings}
      <button class="d-act" onclick={onOpenSettings}>⚙ Settings</button>
    {/if}
    <button class="d-act" onclick={() => revealInFolder(mod.path).catch(() => {})}>
      📂 Reveal
    </button>
  </div>

  <div class="d-meta">
    <div><span>Category</span>{mod.category}{mod.subcategory ? ` · ${mod.subcategory}` : ""}</div>
    <div><span>Size</span><span class="tnum">{sizeLabel}</span></div>
    <div><span>Type</span>{mod.isMap ? "Map" : mod.kind === "dir" ? "Folder" : "Zip"}</div>
    {#if mod.storeItemCount > 0}<div><span>Shop items</span><span class="tnum">{mod.storeItemCount}</span></div>{/if}
    {#if mod.scriptCount > 0}<div><span>Scripts</span><span class="tnum">{mod.scriptCount}</span></div>{/if}
    <div><span>Multiplayer</span>{mod.mpSupported ? "Supported" : "Not marked"}</div>
    {#if organized}<div><span>Library</span>Organized</div>{/if}
  </div>

  <div class="d-section">
    <div class="d-label">Rating</div>
    <div class="d-stars">
      {#each [1, 2, 3, 4, 5] as s (s)}
        <button class="star" class:on={curation.rating >= s} onclick={() => setRating(s)} aria-label={`${s} stars`}>★</button>
      {/each}
    </div>
  </div>

  <div class="d-section">
    <div class="d-label">Tags</div>
    <div class="d-tags">
      {#each tags as t (t)}
        <span class="d-tag">
          <button class="d-tag-name" onclick={() => onFilterTag(t)} title="Filter by this tag">{t}</button>
          <button class="d-tag-x" onclick={() => removeTag(t)} aria-label="Remove tag">✕</button>
        </span>
      {/each}
      <input
        class="d-tag-input"
        placeholder="add tag…"
        bind:value={newTag}
        onkeydown={(e) => e.key === "Enter" && addTag()}
        onblur={addTag}
      />
    </div>
  </div>

  <div class="d-section">
    <div class="d-label">Notes</div>
    <textarea class="d-note" bind:value={noteDraft} onblur={saveNote} placeholder="Personal notes about this mod…"></textarea>
  </div>

  <div class="d-section">
    <div class="d-label">GitHub updates</div>
    <div class="gh-link">
      <input
        class="gh-input"
        placeholder="owner/repo"
        bind:value={repoInput}
        onkeydown={(e) => e.key === "Enter" && linkRepo()}
      />
      <button class="gh-btn" onclick={linkRepo}>{repo ? "Update link" : "Link"}</button>
    </div>
    {#if repo}
      <div class="gh-actions">
        <button class="gh-btn" onclick={checkUpdate} disabled={checking}>
          {checking ? "Checking…" : "Check for update"}
        </button>
        <button class="gh-open" onclick={() => openExternal(`https://github.com/${repo.owner}/${repo.repo}`)}>
          Open repo ↗
        </button>
      </div>
    {/if}
    {#if ghError}<div class="gh-err">{ghError}</div>{/if}
    {#if update}
      <div class="gh-result" class:has={update.hasUpdate}>
        {#if update.hasUpdate}
          <b>Update available:</b> {update.release.tag} (you have {update.current})
        {:else}
          Up to date — latest is {update.release.tag}
        {/if}
        {#if update.release.htmlUrl}
          <button class="gh-open" onclick={() => openExternal(update!.release.htmlUrl!)}>
            View release ↗
          </button>
        {/if}
        {#if update.hasUpdate && update.release.assetUrl}
          <button class="gh-btn" onclick={installUpdate} disabled={installing}>
            {installing ? "Installing…" : "Download & install"}
          </button>
        {/if}
      </div>
    {/if}
  </div>

  {#if mod.dependencies.length > 0}
    <div class="d-section">
      <div class="d-label">Dependencies ({mod.dependencies.length})</div>
      {#each mod.dependencies as dep (dep)}
        <div class="d-dep">
          <span class="d-dep-name tnum">{dep}</span>
          {#if libraryTechNames.has(dep)}
            <span class="d-ok">in library</span>
          {:else}
            <span class="d-missing">missing</span>
          {/if}
        </div>
      {/each}
    </div>
  {/if}

  {#if mod.uniqueType}
    <div class="d-section">
      <div class="d-label">uniqueType</div>
      <div class="d-uniq tnum">{mod.uniqueType}</div>
    </div>
  {/if}

  {#if myConflicts.length > 0}
    <div class="d-section">
      <div class="d-label">Conflicts with the active set ({myConflicts.length})</div>
      {#each myConflicts as c (c.kind + c.name)}
        <div class="d-conf" class:crit={c.severity === "critical"}>
          <span class="d-conf-sev">{c.severity}</span>
          {c.kind} “{c.name}” — with {c.mods.filter((m) => m !== label).join(", ")}
        </div>
      {/each}
    </div>
  {/if}
</aside>

<style>
  .drawer {
    position: fixed;
    z-index: 50;
    top: 0;
    right: 0;
    height: 100vh;
    width: 420px;
    max-width: calc(100vw - 40px);
    overflow-y: auto;
    background: var(--surface-raised);
    border-left: 1px solid var(--border);
    box-shadow: var(--shadow-2);
    padding: 18px;
    scrollbar-width: thin;
  }
  .d-top {
    display: flex;
    gap: 12px;
    align-items: flex-start;
  }
  .d-icon {
    width: 56px;
    height: 56px;
    border-radius: var(--radius);
    object-fit: cover;
    flex: 0 0 auto;
    background: var(--surface);
  }
  .d-icon.ph {
    display: grid;
    place-items: center;
    font-family: var(--font-display);
    font-size: 24px;
    color: var(--on-primary);
    background: linear-gradient(135deg, var(--green-500), var(--green-700));
  }
  .d-head {
    flex: 1 1 auto;
    min-width: 0;
  }
  .d-title {
    font-family: var(--font-display);
    font-size: 18px;
    font-weight: 600;
    line-height: 1.2;
  }
  .d-sub {
    font-size: 12.5px;
    color: var(--text-muted);
    margin-top: 2px;
  }
  .d-tech {
    font-size: 11px;
    color: var(--text-muted);
    opacity: 0.7;
    margin-top: 2px;
  }
  .d-x {
    border: none;
    background: transparent;
    color: var(--text-muted);
    width: 30px;
    height: 30px;
    border-radius: var(--radius-sm);
    font-size: 14px;
  }
  .d-x:hover {
    background: color-mix(in srgb, var(--primary) 12%, transparent);
    color: var(--text);
  }
  .d-actions {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin: 14px 0;
  }
  .d-act {
    border: 1px solid var(--border);
    background: var(--bg);
    color: var(--text-muted);
    padding: 6px 10px;
    border-radius: var(--radius-sm);
    font-size: 12px;
    font-weight: 600;
  }
  .d-act:hover {
    color: var(--text);
  }
  .d-act.on {
    color: var(--primary);
    border-color: color-mix(in srgb, var(--primary) 45%, var(--border));
    background: color-mix(in srgb, var(--primary) 10%, transparent);
  }
  .d-meta {
    display: grid;
    gap: 6px;
    font-size: 12.5px;
    padding: 10px 0;
    border-top: 1px solid var(--border);
  }
  .d-meta > div {
    display: flex;
    justify-content: space-between;
    gap: 12px;
  }
  .d-meta span:first-child {
    color: var(--text-muted);
  }
  .d-meta > div > span:first-child {
    color: var(--text-muted);
  }
  .d-section {
    padding: 12px 0;
    border-top: 1px solid var(--border);
  }
  .d-label {
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--text-muted);
    font-weight: 700;
    margin-bottom: 8px;
  }
  .d-stars {
    display: flex;
    gap: 2px;
  }
  .star {
    border: none;
    background: transparent;
    color: var(--border);
    font-size: 22px;
    line-height: 1;
    padding: 0 2px;
  }
  .star.on {
    color: var(--accent);
  }
  .d-tags {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    align-items: center;
  }
  .d-tag {
    display: inline-flex;
    align-items: center;
    background: color-mix(in srgb, var(--info) 12%, transparent);
    border: 1px solid color-mix(in srgb, var(--info) 30%, var(--border));
    border-radius: 999px;
    overflow: hidden;
  }
  .d-tag-name {
    border: none;
    background: transparent;
    color: var(--info);
    font-size: 12px;
    font-weight: 600;
    padding: 4px 4px 4px 10px;
  }
  .d-tag-x {
    border: none;
    background: transparent;
    color: var(--info);
    font-size: 11px;
    padding: 4px 8px 4px 4px;
    opacity: 0.7;
  }
  .d-tag-x:hover {
    opacity: 1;
  }
  .d-tag-input {
    border: 1px dashed var(--border);
    background: transparent;
    color: var(--text);
    border-radius: 999px;
    padding: 5px 12px;
    font-size: 12px;
    font-family: inherit;
    width: 100px;
  }
  .d-note {
    width: 100%;
    min-height: 70px;
    resize: vertical;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--bg);
    color: var(--text);
    padding: 8px 10px;
    font-family: inherit;
    font-size: 12.5px;
  }
  .d-dep,
  .d-conf {
    display: flex;
    justify-content: space-between;
    gap: 10px;
    font-size: 12.5px;
    padding: 4px 0;
  }
  .d-dep-name {
    color: var(--text);
  }
  .d-ok {
    color: var(--primary);
    font-size: 11.5px;
  }
  .d-missing {
    color: var(--warn);
    font-size: 11.5px;
    font-weight: 600;
  }
  .d-uniq {
    font-size: 12.5px;
    color: var(--warn);
  }
  .gh-link {
    display: flex;
    gap: 6px;
  }
  .gh-input {
    flex: 1 1 auto;
    min-width: 0;
    border: 1px solid var(--border);
    background: var(--bg);
    color: var(--text);
    border-radius: var(--radius-sm);
    padding: 7px 10px;
    font-size: 12.5px;
    font-family: inherit;
  }
  .gh-btn {
    flex: 0 0 auto;
    border: 1px solid var(--border);
    background: var(--bg);
    color: var(--text);
    padding: 7px 12px;
    border-radius: var(--radius-sm);
    font-size: 12.5px;
    font-weight: 600;
  }
  .gh-btn:hover:not(:disabled) {
    border-color: color-mix(in srgb, var(--primary) 45%, var(--border));
    color: var(--primary);
  }
  .gh-actions {
    display: flex;
    gap: 6px;
    margin-top: 8px;
  }
  .gh-open {
    border: none;
    background: transparent;
    color: var(--info);
    font-size: 12px;
    font-weight: 600;
  }
  .gh-open:hover {
    text-decoration: underline;
  }
  .gh-err {
    margin-top: 8px;
    color: var(--danger);
    font-size: 12px;
  }
  .gh-result {
    margin-top: 8px;
    font-size: 12.5px;
    padding: 8px 10px;
    border-radius: var(--radius-sm);
    background: var(--bg);
    display: flex;
    align-items: center;
    gap: 8px;
    flex-wrap: wrap;
  }
  .gh-result.has {
    background: color-mix(in srgb, var(--accent) 12%, transparent);
    color: var(--gold-700);
  }
  .d-conf {
    display: block;
    border-left: 3px solid var(--warn);
    padding: 6px 10px;
    margin-bottom: 6px;
    border-radius: var(--radius-sm);
    background: color-mix(in srgb, var(--warn) 6%, transparent);
    font-size: 12px;
  }
  .d-conf.crit {
    border-left-color: var(--danger);
    background: color-mix(in srgb, var(--danger) 6%, transparent);
  }
  .d-conf-sev {
    font-weight: 700;
    font-size: 10px;
    text-transform: uppercase;
    color: var(--warn);
    margin-right: 6px;
  }
  .d-conf.crit .d-conf-sev {
    color: var(--danger);
  }
</style>
