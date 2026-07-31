// Small, pure formatting helpers shared across components.

/** Human-readable byte size (e.g. "1.2 GB", "512 MB"). */
export function fmtSize(b: number): string {
  if (b >= 1024 ** 3) return (b / 1024 ** 3).toFixed(1) + " GB";
  if (b >= 1024 ** 2) return (b / 1024 ** 2).toFixed(0) + " MB";
  if (b >= 1024) return (b / 1024).toFixed(0) + " KB";
  return b + " B";
}
