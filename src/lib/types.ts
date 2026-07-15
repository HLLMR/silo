// Mirrors the Rust `scan::ModEntry` / `scan::ScanResult` (serde camelCase).

export interface ModEntry {
  techName: string;
  path: string;
  kind: "zip" | "dir";
  size: number;
  mtimeMs: number;

  title: string | null;
  author: string | null;
  version: string | null;
  descVersion: number | null;
  iconFilename: string | null;

  isMap: boolean;
  mapTitle: string | null;
  category: string;
  subcategory: string | null;

  dependencies: string[];
  scriptCount: number;
  registrationCount: number;
  uniqueType: string | null;
  storeItemCount: number;
  mpSupported: boolean;

  ignoredDigitPrefix: boolean;
  error: string | null;
}

export interface ScanResult {
  mods: ModEntry[];
  roots: string[];
  tookMs: number;
  total: number;
}

export interface ScanProgress {
  done: number;
  total: number;
}

export interface CurationRow {
  techName: string;
  favorite: boolean;
  hidden: boolean;
  broken: boolean;
  note: string | null;
}

export interface CategoryOverride {
  techName: string;
  category: string;
  subcategory: string | null;
}
