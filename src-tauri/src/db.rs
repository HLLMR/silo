//! SQLite persistence. First use: a scan cache keyed by path+mtime+size that lets
//! warm rescans skip opening/parsing archives entirely. Real indexed tables (a PK
//! on path) — never the incumbent's `LIKE 'mods_%'` KV scans. Additive migrations;
//! we never wipe the cache on version bumps.

use rusqlite::Connection;
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// A cached scan row: the invalidation key (mtime+size) plus the serialized entry.
pub struct CacheEntry {
    pub mtime_ms: u64,
    pub size: u64,
    pub json: String,
}

/// Open (creating if needed) the Silo database and ensure the schema exists.
pub fn open(db_path: &Path) -> Result<Connection, String> {
    if let Some(dir) = db_path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    conn.execute_batch(
        "PRAGMA journal_mode=WAL;
         CREATE TABLE IF NOT EXISTS mod_cache (
             path      TEXT PRIMARY KEY,
             mtime_ms  INTEGER NOT NULL,
             size      INTEGER NOT NULL,
             json      TEXT NOT NULL
         );",
    )
    .map_err(|e| e.to_string())?;
    Ok(conn)
}

/// Load the entire scan cache into memory (one query) for lock-free lookups
/// during the parallel scan.
pub fn load_cache(conn: &Connection) -> HashMap<String, CacheEntry> {
    let mut map = HashMap::new();
    let Ok(mut stmt) = conn.prepare("SELECT path, mtime_ms, size, json FROM mod_cache") else {
        return map;
    };
    let rows = stmt.query_map([], |r| {
        Ok((
            r.get::<_, String>(0)?,
            r.get::<_, i64>(1)? as u64,
            r.get::<_, i64>(2)? as u64,
            r.get::<_, String>(3)?,
        ))
    });
    if let Ok(rows) = rows {
        for (path, mtime_ms, size, json) in rows.flatten() {
            map.insert(path, CacheEntry { mtime_ms, size, json });
        }
    }
    map
}

/// Upsert freshly-parsed rows in a single transaction.
pub fn upsert_many(conn: &mut Connection, rows: &[(String, u64, u64, String)]) -> Result<(), String> {
    if rows.is_empty() {
        return Ok(());
    }
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    {
        let mut stmt = tx
            .prepare(
                "INSERT INTO mod_cache(path, mtime_ms, size, json) VALUES(?1, ?2, ?3, ?4)
                 ON CONFLICT(path) DO UPDATE SET
                     mtime_ms = excluded.mtime_ms,
                     size     = excluded.size,
                     json     = excluded.json",
            )
            .map_err(|e| e.to_string())?;
        for (path, mtime, size, json) in rows {
            stmt.execute(rusqlite::params![path, *mtime as i64, *size as i64, json])
                .map_err(|e| e.to_string())?;
        }
    }
    tx.commit().map_err(|e| e.to_string())
}

/// Delete cache rows for mods that are no longer present on disk.
pub fn prune_missing(conn: &mut Connection, present: &HashSet<String>) -> Result<usize, String> {
    let existing: Vec<String> = {
        let mut stmt = conn
            .prepare("SELECT path FROM mod_cache")
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([], |r| r.get::<_, String>(0))
            .map_err(|e| e.to_string())?;
        rows.flatten().collect()
    };
    let stale: Vec<&String> = existing.iter().filter(|p| !present.contains(*p)).collect();
    if stale.is_empty() {
        return Ok(0);
    }
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    {
        let mut stmt = tx
            .prepare("DELETE FROM mod_cache WHERE path = ?1")
            .map_err(|e| e.to_string())?;
        for p in &stale {
            stmt.execute([p]).map_err(|e| e.to_string())?;
        }
    }
    tx.commit().map_err(|e| e.to_string())?;
    Ok(stale.len())
}
