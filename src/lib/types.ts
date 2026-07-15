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
  organized: boolean;
  active: boolean;

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
  rating: number;
  note: string | null;
}

export interface TagRow {
  techName: string;
  tag: string;
}

export interface CategoryOverride {
  techName: string;
  category: string;
  subcategory: string | null;
}

export interface ModInput {
  techName: string;
  fileName: string;
  kind: string;
  category: string;
  subcategory: string | null;
}

export interface OrganizeReport {
  changed: number;
  skipped: number;
  errors: string[];
}

export interface Loadout {
  id: number;
  name: string;
  mods: string[];
}

export interface SaveMod {
  modName: string;
  title: string | null;
  version: string | null;
  required: boolean;
  fileHash: string | null;
  isDlc: boolean;
}

export interface Savegame {
  index: number;
  folder: string;
  name: string;
  mapTitle: string | null;
  mods: SaveMod[];
}

export interface ConflictInput {
  techName: string;
  title: string | null;
  path: string;
  kind: string;
}

export interface Conflict {
  severity: "critical" | "warning" | "info";
  kind: string;
  name: string;
  explanation: string;
  mods: string[];
}

export interface GameInfo {
  appId: string;
  exe: string;
  installDir: string;
}

export interface SettingsField {
  id: number;
  label: string;
  kind: "bool" | "int" | "float" | "string";
  value: string;
}

export interface SettingsFile {
  path: string;
  name: string;
  fields: SettingsField[];
  raw: string;
}

export interface SettingsEdit {
  id: number;
  value: string;
}

export interface RepoRow {
  techName: string;
  owner: string;
  repo: string;
}

export interface ReleaseInfo {
  tag: string;
  name: string | null;
  publishedAt: string | null;
  htmlUrl: string | null;
  assetUrl: string | null;
  assetName: string | null;
}

export interface UpdateInfo {
  hasUpdate: boolean;
  current: string;
  release: ReleaseInfo;
}
