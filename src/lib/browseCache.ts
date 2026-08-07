// A tiny in-memory cache of Browse result "views", keyed by the full server-side
// filter + sort combo. It lives OUTSIDE ModBrowser (which remounts on every tab switch)
// so going back to a view you've already fetched is instant — no re-poll. Sort stays
// server-side (globally correct), so it's part of the key. Session-scoped; cleared on reload.
import type { BrowseMod } from "./types";

export interface CachedView {
  results: BrowseMod[];
  total: number;
  lastPageFull: boolean;
}

const cache = new Map<string, CachedView>();

/** Stable key for a filter+sort combo (tags sorted so order doesn't matter). */
export function browseCacheKey(p: {
  category: string;
  tags: string[];
  availableBy: number | null;
  query: string;
  sort: string;
}): string {
  return JSON.stringify({
    c: p.category,
    t: [...p.tags].sort(),
    a: p.availableBy,
    q: p.query.trim(),
    s: p.sort,
  });
}

export function getBrowseView(key: string): CachedView | undefined {
  return cache.get(key);
}

export function setBrowseView(key: string, v: CachedView): void {
  cache.set(key, v);
}
