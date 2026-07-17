// Typed wrappers over the Tauri command surface. The frontend never touches a
// filesystem or zip — it asks the Rust core and listens for progress events.

import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { save, open } from "@tauri-apps/plugin-dialog";
import { revealItemInDir, openPath, openUrl } from "@tauri-apps/plugin-opener";
import type {
  ScanResult,
  ScanProgress,
  CurationRow,
  CategoryOverride,
  TagRow,
  ModInput,
  OrganizeReport,
  Loadout,
  Savegame,
  Conflict,
  ConflictInput,
  GameInfo,
  SettingsFile,
  SettingsEdit,
  RepoRow,
  UpdateInfo,
  GhStatus,
  DeviceCode,
  PollResult,
  BrowseMod,
  BrowsePage,
  SiloStats,
  InstallProgress,
  CatalogUpdate,
  CatalogModDetail,
  CategoryCount,
} from "./types";

export function defaultModsPaths(): Promise<string[]> {
  return invoke<string[]>("default_mods_paths");
}

export function scanMods(roots?: string[]): Promise<ScanResult> {
  return invoke<ScanResult>("scan_mods", { roots: roots ?? null });
}

export function getModIcon(
  path: string,
  kind: string,
  iconFilename: string | null,
): Promise<string | null> {
  return invoke<string | null>("get_mod_icon", { path, kind, iconFilename });
}

export function onScanProgress(
  handler: (p: ScanProgress) => void,
): Promise<UnlistenFn> {
  return listen<ScanProgress>("scan:progress", (e) => handler(e.payload));
}

export function getCuration(): Promise<CurationRow[]> {
  return invoke<CurationRow[]>("get_curation");
}

export function setCuration(row: CurationRow): Promise<void> {
  return invoke("set_curation", { row });
}

export function getTags(): Promise<TagRow[]> {
  return invoke<TagRow[]>("get_tags");
}

export function setTags(techName: string, tags: string[]): Promise<void> {
  return invoke("set_tags", { techName, tags });
}

export function getOverrides(): Promise<CategoryOverride[]> {
  return invoke<CategoryOverride[]>("get_overrides");
}

export function setOverride(row: CategoryOverride): Promise<void> {
  return invoke("set_override", { row });
}

// ── Organize / projection ──
export function applyOrganize(mods: ModInput[]): Promise<OrganizeReport> {
  return invoke<OrganizeReport>("apply_organize", { root: null, mods });
}

export function setActive(active: string[]): Promise<OrganizeReport> {
  return invoke<OrganizeReport>("set_active", { root: null, active });
}

export function flatten(): Promise<OrganizeReport> {
  return invoke<OrganizeReport>("flatten", { root: null });
}

// ── Loadouts ──
export function getLoadouts(): Promise<Loadout[]> {
  return invoke<Loadout[]>("get_loadouts");
}

export function saveLoadout(
  id: number | null,
  name: string,
  mods: string[],
): Promise<number> {
  return invoke<number>("save_loadout", { id, name, mods });
}

export function deleteLoadout(id: number): Promise<void> {
  return invoke("delete_loadout", { id });
}

/** Prompt for a path and export the loadout as a .silo file. Returns false if cancelled. */
export async function exportLoadoutFile(id: number, name: string): Promise<boolean> {
  const path = await save({
    defaultPath: `${name}.silo`,
    filters: [{ name: "Silo loadout", extensions: ["silo"] }],
  });
  if (!path) return false;
  await invoke("export_loadout", { id, path });
  return true;
}

/** Prompt for a .silo file and import it as a new loadout. Returns the new id or null. */
export async function importLoadoutFile(): Promise<number | null> {
  const path = await open({
    multiple: false,
    filters: [{ name: "Silo loadout", extensions: ["silo", "json"] }],
  });
  if (!path || Array.isArray(path)) return null;
  return await invoke<number>("import_loadout", { path });
}

export function getSavegames(): Promise<Savegame[]> {
  return invoke<Savegame[]>("get_savegames");
}

export function backupSavegame(folder: string): Promise<string> {
  return invoke<string>("backup_savegame", { folder });
}

export function detectConflicts(mods: ConflictInput[]): Promise<Conflict[]> {
  return invoke<Conflict[]>("detect_conflicts", { mods });
}

export function revealInFolder(path: string): Promise<void> {
  return revealItemInDir(path);
}

export function openFolder(path: string): Promise<void> {
  return openPath(path);
}

export function openExternal(url: string): Promise<void> {
  return openUrl(url);
}

export function getModRepos(): Promise<RepoRow[]> {
  return invoke<RepoRow[]>("get_mod_repos");
}

export function setModRepo(
  techName: string,
  owner: string,
  repo: string,
): Promise<void> {
  return invoke("set_mod_repo", { techName, owner, repo });
}

export function guessRepo(
  path: string,
  kind: string,
): Promise<RepoRow | null> {
  return invoke<RepoRow | null>("guess_repo", { path, kind });
}

export function checkModUpdate(
  owner: string,
  repo: string,
  current: string,
): Promise<UpdateInfo> {
  return invoke<UpdateInfo>("check_mod_update", { owner, repo, current });
}

// ── GitHub OAuth (device flow) ──
export function ghStatus(): Promise<GhStatus> {
  return invoke<GhStatus>("gh_status");
}
export function ghSetClientId(clientId: string): Promise<void> {
  return invoke("gh_set_client_id", { clientId });
}
export function ghDeviceStart(): Promise<DeviceCode> {
  return invoke<DeviceCode>("gh_device_start");
}
export function ghDevicePoll(deviceCode: string): Promise<PollResult> {
  return invoke<PollResult>("gh_device_poll", { deviceCode });
}
export function ghLogout(): Promise<void> {
  return invoke("gh_logout");
}

/** Download a release .zip and install it in place (backs up the old file). */
export function downloadUpdate(path: string, assetUrl: string): Promise<void> {
  return invoke("download_update", { path, assetUrl });
}

// ── SiloAPI (mod browser / discovery) ──
export function siloapiStatus(): Promise<string> {
  return invoke<string>("siloapi_status");
}

export function siloapiSetBase(base: string): Promise<void> {
  return invoke("siloapi_set_base", { base });
}

export function browseMods(opts: {
  query?: string;
  category?: string;
  limit?: number;
  offset?: number;
}): Promise<BrowsePage> {
  return invoke<BrowsePage>("browse_mods", {
    query: opts.query ?? null,
    category: opts.category ?? null,
    limit: opts.limit ?? null,
    offset: opts.offset ?? null,
  });
}

export function siloapiStats(): Promise<SiloStats> {
  return invoke<SiloStats>("siloapi_stats");
}

/** One mod's full catalog record + every source it was seen on. */
export function siloapiModDetail(id: string): Promise<CatalogModDetail> {
  return invoke<CatalogModDetail>("siloapi_mod_detail", { id });
}

/** Catalog categories with counts, for the Browse filter. */
export function siloapiCategories(): Promise<CategoryCount[]> {
  return invoke<CategoryCount[]>("siloapi_categories");
}

/** Download a browsed mod's .zip into the library. Returns the installed filename.
 *  `source` picks which source to fetch from (the button the user clicked). */
export function installRemoteMod(id: string, source?: string): Promise<string> {
  return invoke<string>("install_remote_mod", { id, source: source ?? null, root: null });
}

/** Per-mod download progress during install. */
export function onInstallProgress(
  handler: (p: InstallProgress) => void,
): Promise<UnlistenFn> {
  return listen<InstallProgress>("install:progress", (e) => handler(e.payload));
}

/** Check the whole library against the catalog in one request (by tech name). */
export function catalogCheckUpdates(
  mods: { techName: string; version?: string }[],
): Promise<CatalogUpdate[]> {
  return invoke<CatalogUpdate[]>("catalog_check_updates", {
    mods: mods.map((m) => ({ techName: m.techName, version: m.version ?? null })),
  });
}

export function detectGame(): Promise<GameInfo | null> {
  return invoke<GameInfo | null>("detect_game");
}

export function launchGame(): Promise<void> {
  return invoke("launch_game");
}

/** Prompt for a path and save text (used by the diagnostics report). */
export async function saveTextFile(
  defaultName: string,
  content: string,
): Promise<boolean> {
  const path = await save({
    defaultPath: defaultName,
    filters: [{ name: "Report", extensions: ["md", "txt"] }],
  });
  if (!path) return false;
  await invoke("save_text", { path, content });
  return true;
}

// ── Mod settings form ──
export function userDirPath(): Promise<string | null> {
  return invoke<string | null>("user_dir_path");
}

export function getConfig(
  path: string,
  paths: string[],
): Promise<Record<string, string>> {
  return invoke<Record<string, string>>("get_config", { path, paths });
}

export function setConfig(
  path: string,
  edits: { path: string; value: string }[],
): Promise<void> {
  return invoke("set_config", { path, edits });
}

export function modsWithSettings(): Promise<string[]> {
  return invoke<string[]>("mods_with_settings");
}

export function getModSettings(modName: string): Promise<SettingsFile[]> {
  return invoke<SettingsFile[]>("get_mod_settings", { modName });
}

export function saveModSettings(
  path: string,
  edits: SettingsEdit[],
): Promise<void> {
  return invoke("save_mod_settings", { path, edits });
}

export function saveModSettingsRaw(path: string, content: string): Promise<void> {
  return invoke("save_mod_settings_raw", { path, content });
}
