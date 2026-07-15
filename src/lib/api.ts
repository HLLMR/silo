// Typed wrappers over the Tauri command surface. The frontend never touches a
// filesystem or zip — it asks the Rust core and listens for progress events.

import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type {
  ScanResult,
  ScanProgress,
  CurationRow,
  CategoryOverride,
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
