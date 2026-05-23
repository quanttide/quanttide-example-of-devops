use std::path::{Path, PathBuf};

pub struct HistoryDb {
    db: rusqlite::Connection,
    repo_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct OperationRecord {
    pub id: i64,
    pub timestamp: String,
    pub action: String,
    pub submodule_name: String,
    pub detail: String,
    pub success: bool,
}

impl HistoryDb {
    pub fn open(repo_root: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let db_dir = repo_root.join(".git").join("kse");
        std::fs::create_dir_all(&db_dir)?;
        let db_path = db_dir.join("history.db");
        let db = rusqlite::Connection::open(&db_path)?;
        let hist = Self {
            db,
            repo_path: repo_root.to_path_buf(),
        };
        hist.initialize()?;
        Ok(hist)
    }

    fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.db.execute_batch(
            "CREATE TABLE IF NOT EXISTS operations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL DEFAULT (datetime('now')),
                action TEXT NOT NULL,
                submodule_name TEXT NOT NULL,
                detail TEXT DEFAULT '',
                success INTEGER NOT NULL DEFAULT 1
            );
            CREATE TABLE IF NOT EXISTS retired_submodules (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                url TEXT DEFAULT '',
                path TEXT DEFAULT '',
                retired_at TEXT NOT NULL DEFAULT (datetime('now')),
                reason TEXT DEFAULT ''
            );",
        )?;
        Ok(())
    }

    pub fn log_operation(
        &self,
        action: &str,
        submodule_name: &str,
        detail: &str,
        success: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.db.execute(
            "INSERT INTO operations (action, submodule_name, detail, success) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![action, submodule_name, detail, success as i32],
        )?;
        Ok(())
    }

    pub fn log_retire(
        &self,
        name: &str,
        url: &str,
        path: &str,
        reason: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.db.execute(
            "INSERT INTO retired_submodules (name, url, path, reason) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![name, url, path, reason],
        )?;
        self.log_operation("retire", name, &format!("退役子模块: {}", reason), true)
    }

    pub fn list_operations(
        &self,
        limit: usize,
        submodule_filter: Option<&str>,
    ) -> Result<Vec<OperationRecord>, Box<dyn std::error::Error>> {
        let (sql, params): (String, Vec<Box<dyn rusqlite::types::ToSql>>) =
            if let Some(name) = submodule_filter {
                (
                    "SELECT id, timestamp, action, submodule_name, detail, success FROM operations WHERE submodule_name = ?1 ORDER BY id DESC LIMIT ?2".into(),
                    vec![Box::new(name.to_string()), Box::new(limit as i64)],
                )
            } else {
                (
                    "SELECT id, timestamp, action, submodule_name, detail, success FROM operations ORDER BY id DESC LIMIT ?1".into(),
                    vec![Box::new(limit as i64)],
                )
            };

        let mut stmt = self.db.prepare(&sql)?;
        let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        let rows = stmt.query_map(param_refs.as_slice(), |row| {
            Ok(OperationRecord {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                action: row.get(2)?,
                submodule_name: row.get(3)?,
                detail: row.get(4)?,
                success: row.get::<_, i32>(5)? != 0,
            })
        })?;

        let mut records = Vec::new();
        for row in rows {
            records.push(row?);
        }
        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_db() -> (tempfile::TempDir, HistoryDb) {
        let tmp = tempfile::tempdir().unwrap();
        let db = HistoryDb::open(tmp.path()).unwrap();
        (tmp, db)
    }

    #[test]
    fn test_open_creates_db_file() {
        let tmp = tempfile::tempdir().unwrap();
        HistoryDb::open(tmp.path()).unwrap();
        let db_path = tmp.path().join(".git").join("kse").join("history.db");
        assert!(db_path.exists(), "DB file should be created");
    }

    #[test]
    fn test_log_and_list_operation() {
        let (_tmp, db) = temp_db();
        db.log_operation("update", "lib-a", "updated to latest", true).unwrap();
        let records = db.list_operations(10, None).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].action, "update");
        assert_eq!(records[0].submodule_name, "lib-a");
        assert!(records[0].success);
    }

    #[test]
    fn test_log_failure() {
        let (_tmp, db) = temp_db();
        db.log_operation("sync", "lib-b", "network error", false).unwrap();
        let records = db.list_operations(10, None).unwrap();
        assert_eq!(records.len(), 1);
        assert!(!records[0].success);
    }

    #[test]
    fn test_list_operations_limit() {
        let (_tmp, db) = temp_db();
        for i in 0..5 {
            db.log_operation("update", &format!("lib-{}", i), "", true).unwrap();
        }
        let all = db.list_operations(10, None).unwrap();
        assert_eq!(all.len(), 5);
        let limited = db.list_operations(2, None).unwrap();
        assert_eq!(limited.len(), 2);
    }

    #[test]
    fn test_list_operations_filter_by_submodule() {
        let (_tmp, db) = temp_db();
        db.log_operation("update", "lib-a", "", true).unwrap();
        db.log_operation("sync", "lib-b", "", true).unwrap();
        db.log_operation("update", "lib-a", "", true).unwrap();

        let records = db.list_operations(10, Some("lib-a")).unwrap();
        assert_eq!(records.len(), 2);
        for r in &records {
            assert_eq!(r.submodule_name, "lib-a");
        }
    }

    #[test]
    fn test_list_operations_filter_no_match() {
        let (_tmp, db) = temp_db();
        db.log_operation("update", "lib-a", "", true).unwrap();
        let records = db.list_operations(10, Some("nonexistent")).unwrap();
        assert_eq!(records.len(), 0);
    }

    #[test]
    fn test_log_retire() {
        let (_tmp, db) = temp_db();
        db.log_retire("old-lib", "https://example.com/old.git", "libs/old", "no longer needed")
            .unwrap();

        let ops = db.list_operations(10, None).unwrap();
        assert_eq!(ops.len(), 1);
        assert_eq!(ops[0].action, "retire");
        assert_eq!(ops[0].submodule_name, "old-lib");

        let count: i64 = db
            .db
            .query_row("SELECT COUNT(*) FROM retired_submodules WHERE name = ?1", ["old-lib"], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_empty_history() {
        let (_tmp, db) = temp_db();
        let records = db.list_operations(10, None).unwrap();
        assert!(records.is_empty());
    }

    #[test]
    fn test_multiple_operations_order() {
        let (_tmp, db) = temp_db();
        db.log_operation("add", "a", "", true).unwrap();
        db.log_operation("update", "b", "", true).unwrap();
        db.log_operation("retire", "c", "", true).unwrap();

        let records = db.list_operations(10, None).unwrap();
        assert_eq!(records.len(), 3);
        assert_eq!(records[0].action, "retire");
        assert_eq!(records[2].action, "add");
    }
}

