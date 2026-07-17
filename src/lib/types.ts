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

export interface GhStatus {
  clientId: string | null;
  user: string | null;
  builtin: boolean;
}

export interface DeviceCode {
  deviceCode: string;
  userCode: string;
  verificationUri: string;
  interval: number;
  expiresIn: number;
}

export interface PollResult {
  status: "ok" | "pending" | "slow_down" | "expired" | "denied" | "error";
  error: string | null;
}

// ── SiloAPI (mod browser) ──

/** One place a mod can be got from. The API decides `installable` — ModHub's CDN
 *  blocks direct downloads and Nexus gates them, so those open their site instead. */
export interface ModSourceOption {
  source: string;
  sourceUrl: string;
  version: string | null;
  installable: boolean;
  downloadUrl: string | null;
}

export interface BrowseMod {
  id: string;
  techName: string | null;
  title: string;
  author: string | null;
  category: string | null;
  description: string | null;
  imageUrl: string | null;
  latestVersion: string | null;
  trustScore: number | null;
  updatedAt: string | null;
  sources: ModSourceOption[];
  pageUrl: string | null;
}

/** A page of catalog results + how many match the filter overall. */
export interface BrowsePage {
  mods: BrowseMod[];
  total: number;
}

export interface SiloStats {
  mods: number;
  sources: number;
}

export interface InstallProgress {
  id: string;
  done: number;
  total: number | null;
}

export interface CatalogUpdate {
  techName: string;
  latest: string | null;
  hasUpdate: boolean;
  downloadUrl: string | null;
  source: string | null;
}

/** The detail record is a BrowseMod plus its registrations; sources are the same shape. */
export type CatalogModDetail = BrowseMod;

export interface CategoryCount {
  category: string;
  count: number;
}
