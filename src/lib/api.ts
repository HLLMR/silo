// Typed wrappers over the Tauri command surface. The frontend never touches a
// filesystem or zip — it asks the Rust core and listens for progress events.

import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type {
  ScanResult,
  ScanProgress,
  CurationRow,
  CategoryOverride,
  ModInput,
  OrganizeReport,
  Loadout,
  Savegame,
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

export function getSavegames(): Promise<Savegame[]> {
  return invoke<Savegame[]>("get_savegames");
}
