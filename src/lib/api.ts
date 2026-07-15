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

export function checkModUpdate(
  owner: string,
  repo: string,
  current: string,
): Promise<UpdateInfo> {
  return invoke<UpdateInfo>("check_mod_update", { owner, repo, current });
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
