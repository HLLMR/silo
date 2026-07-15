//! SQLite persistence. First use: a scan cache keyed by path+mtime+size that lets
//! warm rescans skip opening/parsing archives entirely. Real indexed tables (a PK
//! on path) — never the incumbent's `LIKE 'mods_%'` KV scans. Additive migrations;
//! we never wipe the cache on version bumps.

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// A cached scan row: the invalidation key (mtime+size) plus the serialized entry.
pub struct CacheEntry {
    pub mtime_ms: u64,
    pub size: u64,
    pub json: String,
}

/// User curation flags, keyed by mod tech-name (stable across moves/versions).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurationRow {
    pub tech_name: String,
    pub favorite: bool,
    pub hidden: bool,
    pub broken: bool,
    pub note: Option<String>,
}

/// A manual category reassignment, keyed by tech-name.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryOverride {
    pub tech_name: String,
    pub category: String,
    pub subcategory: Option<String>,
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
         );
         CREATE TABLE IF NOT EXISTS curation (
             tech_name TEXT PRIMARY KEY,
             favorite  INTEGER NOT NULL DEFAULT 0,
             hidden    INTEGER NOT NULL DEFAULT 0,
             broken    INTEGER NOT NULL DEFAULT 0,
             note      TEXT
         );
         CREATE TABLE IF NOT EXISTS category_override (
             tech_name   TEXT PRIMARY KEY,
             category    TEXT NOT NULL,
             subcategory TEXT
         );",
    )
    .map_err(|e| e.to_string())?;
    Ok(conn)
}

/// Load all curation rows.
pub fn load_curation(conn: &Connection) -> Vec<CurationRow> {
    let Ok(mut stmt) =
        conn.prepare("SELECT tech_name, favorite, hidden, broken, note FROM curation")
    else {
        return Vec::new();
    };
    let rows = stmt.query_map([], |r| {
        Ok(CurationRow {
            tech_name: r.get(0)?,
            favorite: r.get::<_, i64>(1)? != 0,
            hidden: r.get::<_, i64>(2)? != 0,
            broken: r.get::<_, i64>(3)? != 0,
            note: r.get(4)?,
        })
    });
    rows.map(|r| r.flatten().collect()).unwrap_or_default()
}

/// Upsert one curation row. If all flags are false and note is empty, delete it.
pub fn set_curation(conn: &Connection, row: &CurationRow) -> Result<(), String> {
    let empty = !row.favorite && !row.hidden && !row.broken && row.note.as_deref().unwrap_or("").is_empty();
    if empty {
        conn.execute("DELETE FROM curation WHERE tech_name = ?1", [&row.tech_name])
            .map_err(|e| e.to_string())?;
        return Ok(());
    }
    conn.execute(
        "INSERT INTO curation(tech_name, favorite, hidden, broken, note)
         VALUES(?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(tech_name) DO UPDATE SET
             favorite = excluded.favorite,
             hidden   = excluded.hidden,
             broken   = excluded.broken,
             note     = excluded.note",
        rusqlite::params![
            row.tech_name,
            row.favorite as i64,
            row.hidden as i64,
            row.broken as i64,
            row.note,
        ],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// Load all category overrides.
pub fn load_overrides(conn: &Connection) -> Vec<CategoryOverride> {
    let Ok(mut stmt) =
        conn.prepare("SELECT tech_name, category, subcategory FROM category_override")
    else {
        return Vec::new();
    };
    let rows = stmt.query_map([], |r| {
        Ok(CategoryOverride {
            tech_name: r.get(0)?,
            category: r.get(1)?,
            subcategory: r.get(2)?,
        })
    });
    rows.map(|r| r.flatten().collect()).unwrap_or_default()
}

/// Upsert (or clear) a category override.
pub fn set_override(conn: &Connection, row: &CategoryOverride) -> Result<(), String> {
    if row.category.is_empty() {
        conn.execute("DELETE FROM category_override WHERE tech_name = ?1", [&row.tech_name])
            .map_err(|e| e.to_string())?;
        return Ok(());
    }
    conn.execute(
        "INSERT INTO category_override(tech_name, category, subcategory)
         VALUES(?1, ?2, ?3)
         ON CONFLICT(tech_name) DO UPDATE SET
             category = excluded.category,
             subcategory = excluded.subcategory",
        rusqlite::params![row.tech_name, row.category, row.subcategory],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
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
