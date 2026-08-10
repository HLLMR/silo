// Pure helpers shared across the Browse-tab components (card grid + detail drawer).

import type { CatalogModDetail } from "./types";
import { catalogImage } from "./api";

// Source-CDN hosts that gate hotlinking behind a Referer a browser <img> can't set —
// these must go through the Rust proxy (which adds the referer and disk-caches).
const REFERER_GATED = /(?:giants-software\.com|farming-simulator\.com|nexusmods\.com|githubusercontent\.com|github\.com)/i;

// Image loading. SiloAPI now caches most catalog images server-side and hands back an
// `imageUrl` on its own host (referer-free, HTTP-cacheable) — those load DIRECTLY as a
// plain <img> (CSP allows silo-api). Only the not-yet-cached tail still points at a
// source CDN; those go through the Rust proxy, which sets the referer and returns a
// data: URL. Dedupe in-session so paging back doesn't refetch.
const imageCache = new Map<string, Promise<string>>();
export function loadCatalogImage(url: string): Promise<string> {
  // Cached SiloAPI URL (or any non-gated host): the browser can load and cache it itself.
  if (!REFERER_GATED.test(url)) return Promise.resolve(url);
  let p = imageCache.get(url);
  if (!p) {
    p = catalogImage(url).catch(() => "");
    imageCache.set(url, p);
  }
  return p;
}

export const SOURCE_LABEL: Record<string, string> = {
  github: "GitHub",
  modhub: "ModHub",
  nexus: "Nexus Mods",
  kingmods: "KingMods",
};

// Compact codes for the card's source buttons, where space is tight.
export const SOURCE_SHORT: Record<string, string> = {
  github: "GH",
  modhub: "MH",
  nexus: "Nexus",
  kingmods: "KM",
};

export const shortLabel = (s: string): string => SOURCE_SHORT[s] ?? s;
export const label = (s: string): string => SOURCE_LABEL[s] ?? s;

/** Why a source can't be installed directly — shown on hover and in the drawer. */
export function gatedReason(source: string): string {
  if (source === "modhub")
    return "ModHub blocks downloads from outside its website, so Silo can't install this for you. Opens the mod page.";
  if (source === "nexus")
    return "Nexus requires downloads to go through its own site. Opens the mod page.";
  return "This source doesn't allow direct downloads. Opens the mod page.";
}

export function fmtMB(bytes: number): string {
  return (bytes / (1024 * 1024)).toFixed(1);
}

/** Compact download counts: 980, 12k, 1.3M. */
export function fmtCount(n: number): string {
  if (n < 1000) return `${n}`;
  if (n < 1_000_000) return `${(n / 1000).toFixed(n < 10_000 ? 1 : 0)}k`;
  return `${(n / 1_000_000).toFixed(1)}M`;
}

/** Pull owner/repo out of a github.com source URL, for the interactive card. */
export function parseRepo(url: string): { owner: string; repo: string } | null {
  const m = url.match(/github\.com\/([^/]+)\/([^/?#]+)/i);
  if (!m) return null;
  return { owner: m[1], repo: m[2].replace(/\.git$/i, "") };
}

/** Pull the numeric Nexus mod id out of a source URL (…/mods/12345). */
export function parseNexusId(url: string): number | null {
  const m = url.match(/\/mods\/(\d+)/);
  return m ? Number(m[1]) : null;
}

/** Whether a "Read more" affordance is worth showing: an ingested full body, or a
 *  summary long enough that a dedicated modal helps. */
export function canExpand(d: CatalogModDetail): boolean {
  return !!d.descriptionFull || (d.description != null && d.description.length > 160);
}
