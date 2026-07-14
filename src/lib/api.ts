// Typed wrappers over the Tauri command surface. The frontend never touches a
// filesystem or zip — it asks the Rust core and listens for progress events.

import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { ScanResult, ScanProgress } from "./types";

export function defaultModsPaths(): Promise<string[]> {
  return invoke<string[]>("default_mods_paths");
}

export function scanMods(roots?: string[]): Promise<ScanResult> {
  return invoke<ScanResult>("scan_mods", { roots: roots ?? null });
}

export function onScanProgress(
  handler: (p: ScanProgress) => void,
): Promise<UnlistenFn> {
  return listen<ScanProgress>("scan:progress", (e) => handler(e.payload));
}
