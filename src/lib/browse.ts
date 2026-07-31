// Pure helpers shared across the Browse-tab components (card grid + detail drawer).

import type { CatalogModDetail } from "./types";

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

/** Whether a "Read more" affordance is worth showing: a long summary, an ingested
 *  full body, or a Nexus source we can pull the full body from live. */
export function canExpand(d: CatalogModDetail): boolean {
  return (
    !!d.descriptionFull ||
    (d.description != null && d.description.length > 160) ||
    d.sources.some((s) => s.source === "nexus" && parseNexusId(s.sourceUrl))
  );
}
