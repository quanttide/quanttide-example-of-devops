use std::path::Path;

use crate::model::release::{FileStorage, ReleaseStatus, ReleaseRecord, Storage};

pub fn run(repo_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let storage = FileStorage::new(repo_path);
    let mut records = storage.list();
    if records.is_empty() {
        println!("当前无发布记录");
        return Ok(String::new());
    }

    records.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    let staged: Vec<&ReleaseRecord> = records.iter().filter(|r| r.status == ReleaseStatus::Staged).collect();
    let published: Vec<&ReleaseRecord> = records.iter().filter(|r| r.status == ReleaseStatus::Published).collect();

    println!("发布状态报告");
    println!("{}", "-".repeat(40));
    println!("待发布: {}", staged.len());
    for r in &staged {
        println!("  {} (尝试: {})", r.version, &r.id[..8]);
    }
    println!("已发布: {}", published.len());
    for r in &published {
        println!("  {} (尝试: {})", r.version, &r.id[..8]);
    }
    println!();

    println!("最新发布:");
    for r in records.iter().take(5) {
        let status_str = match r.status {
            ReleaseStatus::Staged => "Staged",
            ReleaseStatus::Published => "Published",
            ReleaseStatus::Cancelled => "Cancelled",
            ReleaseStatus::Retired => "Retired",
        };
        println!("  {:<25} {:<12} {}", r.version, status_str, r.updated_at);
    }

    Ok(records.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::release::{ReleaseStatus, Storage};

    fn make_record(version: &str, status: ReleaseStatus) -> ReleaseRecord {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            .to_string();
        ReleaseRecord {
            id: uuid::Uuid::new_v4().to_string(),
            version: version.to_string(),
            status,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    #[test]
    fn test_status_empty() {
        let dir = tempfile::tempdir().unwrap();
        let result = run(dir.path()).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_status_with_records() {
        let dir = tempfile::tempdir().unwrap();
        let mut s = FileStorage::new(dir.path());
        s.save(&make_record("v1.0.0", ReleaseStatus::Staged)).unwrap();
        s.save(&make_record("v2.0.0", ReleaseStatus::Published)).unwrap();

        let result = run(dir.path()).unwrap();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_status_multiple_staged() {
        let dir = tempfile::tempdir().unwrap();
        let mut s = FileStorage::new(dir.path());
        s.save(&make_record("v1.0.0", ReleaseStatus::Staged)).unwrap();
        s.save(&make_record("v2.0.0", ReleaseStatus::Staged)).unwrap();
        s.save(&make_record("v3.0.0", ReleaseStatus::Published)).unwrap();

        let result = run(dir.path()).unwrap();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_status_prints_output() {
        let dir = tempfile::tempdir().unwrap();
        let mut s = FileStorage::new(dir.path());
        s.save(&make_record("v1.0.0", ReleaseStatus::Published)).unwrap();
        s.save(&make_record("v0.5.0-rc.1", ReleaseStatus::Staged)).unwrap();

        let result = run(dir.path());
        assert!(result.is_ok());
    }
}
