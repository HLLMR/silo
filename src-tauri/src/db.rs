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
    #[serde(default)]
    pub rating: i64,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagRow {
    pub tech_name: String,
    pub tag: String,
}

/// A mod linked to its GitHub repo for update-checking.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoRow {
    pub tech_name: String,
    pub owner: String,
    pub repo: String,
}

/// A manual category reassignment, keyed by tech-name.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryOverride {
    pub tech_name: String,
    pub category: String,
    pub subcategory: Option<String>,
}

/// Manifest row: a mod Silo has moved into `mods/archive/<category>/`. `active`
/// means it's currently projected (linked) into the flat root for the game.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizedRow {
    pub tech_name: String,
    pub file_name: String,
    pub kind: String,
    pub category: String,
    pub subcategory: Option<String>,
    pub active: bool,
}

/// A named, saved set of active mods.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Loadout {
    pub id: i64,
    pub name: String,
    pub mods: Vec<String>,
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
         );
         CREATE TABLE IF NOT EXISTS organized (
             tech_name   TEXT PRIMARY KEY,
             file_name   TEXT NOT NULL,
             kind        TEXT NOT NULL,
             category    TEXT NOT NULL,
             subcategory TEXT,
             active      INTEGER NOT NULL DEFAULT 0
         );
         CREATE TABLE IF NOT EXISTS loadout (
             id   INTEGER PRIMARY KEY AUTOINCREMENT,
             name TEXT NOT NULL
         );
         CREATE TABLE IF NOT EXISTS loadout_mod (
             loadout_id INTEGER NOT NULL,
             tech_name  TEXT NOT NULL,
             PRIMARY KEY (loadout_id, tech_name)
         );
         CREATE TABLE IF NOT EXISTS mod_tag (
             tech_name TEXT NOT NULL,
             tag       TEXT NOT NULL,
             PRIMARY KEY (tech_name, tag)
         );
         CREATE TABLE IF NOT EXISTS mod_repo (
             tech_name TEXT PRIMARY KEY,
             owner     TEXT NOT NULL,
             repo      TEXT NOT NULL
         );",
    )
    .map_err(|e| e.to_string())?;

    // Additive migrations (ignore "duplicate column" on existing DBs).
    let _ = conn.execute(
        "ALTER TABLE curation ADD COLUMN rating INTEGER NOT NULL DEFAULT 0",
        [],
    );
    Ok(conn)
}

/// Load all tags, grouped later by the caller.
pub fn load_tags(conn: &Connection) -> Vec<TagRow> {
    let Ok(mut stmt) = conn.prepare("SELECT tech_name, tag FROM mod_tag ORDER BY tag COLLATE NOCASE")
    else {
        return Vec::new();
    };
    let rows = stmt.query_map([], |r| {
        Ok(TagRow { tech_name: r.get(0)?, tag: r.get(1)? })
    });
    rows.map(|r| r.flatten().collect()).unwrap_or_default()
}

/// Load all mod→repo links.
pub fn load_repos(conn: &Connection) -> Vec<RepoRow> {
    let Ok(mut stmt) = conn.prepare("SELECT tech_name, owner, repo FROM mod_repo") else {
        return Vec::new();
    };
    let rows = stmt.query_map([], |r| {
        Ok(RepoRow { tech_name: r.get(0)?, owner: r.get(1)?, repo: r.get(2)? })
    });
    rows.map(|r| r.flatten().collect()).unwrap_or_default()
}

/// Set (or clear, when owner is empty) a mod's GitHub repo link.
pub fn set_repo(conn: &Connection, tech_name: &str, owner: &str, repo: &str) -> Result<(), String> {
    if owner.trim().is_empty() || repo.trim().is_empty() {
        conn.execute("DELETE FROM mod_repo WHERE tech_name = ?1", [tech_name])
            .map_err(|e| e.to_string())?;
        return Ok(());
    }
    conn.execute(
        "INSERT INTO mod_repo(tech_name, owner, repo) VALUES(?1, ?2, ?3)
         ON CONFLICT(tech_name) DO UPDATE SET owner = excluded.owner, repo = excluded.repo",
        rusqlite::params![tech_name, owner.trim(), repo.trim()],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// Replace all tags for a mod.
pub fn set_tags(conn: &mut Connection, tech_name: &str, tags: &[String]) -> Result<(), String> {
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    tx.execute("DELETE FROM mod_tag WHERE tech_name = ?1", [tech_name])
        .map_err(|e| e.to_string())?;
    {
        let mut stmt = tx
            .prepare("INSERT OR IGNORE INTO mod_tag(tech_name, tag) VALUES(?1, ?2)")
            .map_err(|e| e.to_string())?;
        for t in tags {
            let t = t.trim();
            if !t.is_empty() {
                stmt.execute(rusqlite::params![tech_name, t]).map_err(|e| e.to_string())?;
            }
        }
    }
    tx.commit().map_err(|e| e.to_string())
}

/// Load all saved loadouts with their mod lists.
pub fn load_loadouts(conn: &Connection) -> Vec<Loadout> {
    let Ok(mut stmt) = conn.prepare("SELECT id, name FROM loadout ORDER BY name COLLATE NOCASE")
    else {
        return Vec::new();
    };
    let heads: Vec<(i64, String)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))
        .map(|r| r.flatten().collect())
        .unwrap_or_default();

    heads
        .into_iter()
        .map(|(id, name)| {
            let mods = conn
                .prepare("SELECT tech_name FROM loadout_mod WHERE loadout_id = ?1")
                .and_then(|mut s| {
                    s.query_map([id], |r| r.get::<_, String>(0))
                        .map(|rows| rows.flatten().collect::<Vec<_>>())
                })
                .unwrap_or_default();
            Loadout { id, name, mods }
        })
        .collect()
}

/// Create (id None) or update (id Some) a loadout; returns its id.
pub fn save_loadout(
    conn: &mut Connection,
    id: Option<i64>,
    name: &str,
    mods: &[String],
) -> Result<i64, String> {
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    let id = match id {
        Some(id) => {
            tx.execute("UPDATE loadout SET name = ?2 WHERE id = ?1", rusqlite::params![id, name])
                .map_err(|e| e.to_string())?;
            tx.execute("DELETE FROM loadout_mod WHERE loadout_id = ?1", [id])
                .map_err(|e| e.to_string())?;
            id
        }
        None => {
            tx.execute("INSERT INTO loadout(name) VALUES(?1)", [name])
                .map_err(|e| e.to_string())?;
            tx.last_insert_rowid()
        }
    };
    {
        let mut stmt = tx
            .prepare("INSERT OR IGNORE INTO loadout_mod(loadout_id, tech_name) VALUES(?1, ?2)")
            .map_err(|e| e.to_string())?;
        for m in mods {
            stmt.execute(rusqlite::params![id, m]).map_err(|e| e.to_string())?;
        }
    }
    tx.commit().map_err(|e| e.to_string())?;
    Ok(id)
}

pub fn delete_loadout(conn: &mut Connection, id: i64) -> Result<(), String> {
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    tx.execute("DELETE FROM loadout_mod WHERE loadout_id = ?1", [id])
        .map_err(|e| e.to_string())?;
    tx.execute("DELETE FROM loadout WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;
    tx.commit().map_err(|e| e.to_string())
}

/// Load the organize manifest.
pub fn load_organized(conn: &Connection) -> Vec<OrganizedRow> {
    let Ok(mut stmt) = conn
        .prepare("SELECT tech_name, file_name, kind, category, subcategory, active FROM organized")
    else {
        return Vec::new();
    };
    let rows = stmt.query_map([], |r| {
        Ok(OrganizedRow {
            tech_name: r.get(0)?,
            file_name: r.get(1)?,
            kind: r.get(2)?,
            category: r.get(3)?,
            subcategory: r.get(4)?,
            active: r.get::<_, i64>(5)? != 0,
        })
    });
    rows.map(|r| r.flatten().collect()).unwrap_or_default()
}

pub fn upsert_organized(conn: &Connection, row: &OrganizedRow) -> Result<(), String> {
    conn.execute(
        "INSERT INTO organized(tech_name, file_name, kind, category, subcategory, active)
         VALUES(?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(tech_name) DO UPDATE SET
             file_name = excluded.file_name, kind = excluded.kind,
             category = excluded.category, subcategory = excluded.subcategory,
             active = excluded.active",
        rusqlite::params![
            row.tech_name,
            row.file_name,
            row.kind,
            row.category,
            row.subcategory,
            row.active as i64
        ],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn set_organized_active(conn: &Connection, tech_name: &str, active: bool) -> Result<(), String> {
    conn.execute(
        "UPDATE organized SET active = ?2 WHERE tech_name = ?1",
        rusqlite::params![tech_name, active as i64],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn delete_organized(conn: &Connection, tech_name: &str) -> Result<(), String> {
    conn.execute("DELETE FROM organized WHERE tech_name = ?1", [tech_name])
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Load all curation rows.
pub fn load_curation(conn: &Connection) -> Vec<CurationRow> {
    let Ok(mut stmt) =
        conn.prepare("SELECT tech_name, favorite, hidden, broken, note, rating FROM curation")
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
            rating: r.get::<_, i64>(5).unwrap_or(0),
        })
    });
    rows.map(|r| r.flatten().collect()).unwrap_or_default()
}

/// Upsert one curation row. If all flags are false and note is empty, delete it.
pub fn set_curation(conn: &Connection, row: &CurationRow) -> Result<(), String> {
    let empty = !row.favorite
        && !row.hidden
        && !row.broken
        && row.rating == 0
        && row.note.as_deref().unwrap_or("").is_empty();
    if empty {
        conn.execute("DELETE FROM curation WHERE tech_name = ?1", [&row.tech_name])
            .map_err(|e| e.to_string())?;
        return Ok(());
    }
    conn.execute(
        "INSERT INTO curation(tech_name, favorite, hidden, broken, note, rating)
         VALUES(?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(tech_name) DO UPDATE SET
             favorite = excluded.favorite,
             hidden   = excluded.hidden,
             broken   = excluded.broken,
             note     = excluded.note,
             rating   = excluded.rating",
        rusqlite::params![
            row.tech_name,
            row.favorite as i64,
            row.hidden as i64,
            row.broken as i64,
            row.note,
            row.rating,
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
