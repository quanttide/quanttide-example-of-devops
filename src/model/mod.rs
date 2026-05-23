use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitHash(pub String);

impl std::fmt::Display for CommitHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0[..self.0.len().min(7)])
    }
}

impl Default for CommitHash {
    fn default() -> Self {
        Self(String::from("0000000000000000000000000000000000000000"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubmoduleStatus {
    Clean,
    AheadOfParent,
    BehindRemote,
    Detached,
    Dirty,
    Orphaned,
    Uninitialized,
}

impl SubmoduleStatus {
    pub fn priority(&self) -> u8 {
        match self {
            Self::Dirty => 0,
            Self::Orphaned => 1,
            Self::Detached => 2,
            Self::Uninitialized => 3,
            Self::BehindRemote => 4,
            Self::AheadOfParent => 5,
            Self::Clean => 6,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Submodule {
    pub name: String,
    pub path: PathBuf,
    pub url: String,
    pub tracked_branch: String,
    pub parent_pointer: CommitHash,
    pub local_head: CommitHash,
    pub remote_head: CommitHash,
    pub status: SubmoduleStatus,
}

#[derive(Debug, Clone)]
pub struct RepoState {
    pub root_path: PathBuf,
    pub submodules: Vec<Submodule>,
    pub total: usize,
    pub clean_count: usize,
    pub needs_attention: Vec<String>,
}

impl RepoState {
    pub fn scan(root: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let gitmodules_path = root.join(".gitmodules");
        if !gitmodules_path.exists() {
            return Ok(RepoState {
                root_path: root.to_path_buf(),
                submodules: vec![],
                total: 0,
                clean_count: 0,
                needs_attention: vec![],
            });
        }

        let repo = match git2::Repository::open(root) {
            Ok(r) => r,
            Err(e) => return Err(format!("无法打开 Git 仓库 '{}': {}", root.display(), e).into()),
        };
        let mut submodules = Vec::new();

        let mut git_submodules = repo.submodules()?;
        git_submodules.sort_by(|a, b| a.name().cmp(b.name()));

        for sm in &git_submodules {
            let name = sm.name().unwrap_or("unknown").to_string();
            let sm_path = sm.path();
            let full_sm_path = root.join(sm_path);
            let url = sm.url().unwrap_or("").to_string();
            let branch = sm.branch().unwrap_or("main").to_string();

            let raw_status = sm.status(false)?;
            let is_uninitialized =
                raw_status.contains(git2::SubmoduleStatus::WD_UNINITIALIZED);
            let is_dirty = raw_status.contains(git2::SubmoduleStatus::WD_DIRTY);

            // 父仓库记录的 commit
            let parent_pointer = CommitHash(sm.head_id().to_string());

            // 子模块本地 HEAD 和远程 HEAD
            let (local_head, remote_head, is_detached) = if is_uninitialized {
                (CommitHash::default(), CommitHash::default(), false)
            } else {
                match git2::Repository::open(&full_sm_path) {
                    Ok(sub_repo) => {
                        let local = sub_repo
                            .head()
                            .ok()
                            .and_then(|r| r.target())
                            .map(|o| CommitHash(o.to_string()))
                            .unwrap_or_default();

                        let detached = sub_repo
                            .head()
                            .ok()
                            .map(|r| !r.is_branch())
                            .unwrap_or(false);

                        let remote = sub_repo
                            .find_reference(&format!("refs/remotes/origin/{}", branch))
                            .ok()
                            .and_then(|r| r.target())
                            .map(|o| CommitHash(o.to_string()))
                            .unwrap_or_default();

                        (local, remote, detached)
                    }
                    Err(_) => (CommitHash::default(), CommitHash::default(), false),
                }
            };

            let status = if is_uninitialized {
                SubmoduleStatus::Uninitialized
            } else if is_dirty {
                SubmoduleStatus::Dirty
            } else if is_detached {
                SubmoduleStatus::Detached
            } else if local_head == parent_pointer && local_head == remote_head {
                SubmoduleStatus::Clean
            } else if parent_pointer != local_head && local_head == remote_head {
                SubmoduleStatus::AheadOfParent
            } else if remote_head != local_head {
                SubmoduleStatus::BehindRemote
            } else {
                SubmoduleStatus::Clean
            };

            submodules.push(Submodule {
                name,
                path: sm_path.to_path_buf(),
                url,
                tracked_branch: branch,
                parent_pointer,
                local_head,
                remote_head,
                status,
            });
        }

        let total = submodules.len();
        let clean_count = submodules
            .iter()
            .filter(|s| s.status == SubmoduleStatus::Clean)
            .count();
        let needs_attention: Vec<String> = submodules
            .iter()
            .filter(|s| s.status != SubmoduleStatus::Clean)
            .map(|s| s.name.clone())
            .collect();

        Ok(RepoState {
            root_path: root.to_path_buf(),
            submodules,
            total,
            clean_count,
            needs_attention,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_priority_ordering() {
        assert!(SubmoduleStatus::Dirty.priority() < SubmoduleStatus::Clean.priority());
        assert!(SubmoduleStatus::Orphaned.priority() < SubmoduleStatus::BehindRemote.priority());
        assert!(SubmoduleStatus::Detached.priority() < SubmoduleStatus::AheadOfParent.priority());
        assert!(SubmoduleStatus::Uninitialized.priority() < SubmoduleStatus::Clean.priority());
    }

    #[test]
    fn test_clean_is_lowest_priority() {
        let statuses = [
            SubmoduleStatus::Dirty,
            SubmoduleStatus::Orphaned,
            SubmoduleStatus::Detached,
            SubmoduleStatus::Uninitialized,
            SubmoduleStatus::BehindRemote,
            SubmoduleStatus::AheadOfParent,
        ];
        for s in &statuses {
            assert!(s.priority() < SubmoduleStatus::Clean.priority());
        }
    }

    #[test]
    fn test_commit_hash_display_truncates() {
        let hash = CommitHash("abcdef1234567890".to_string());
        let display = format!("{}", hash);
        assert_eq!(display.len(), 7);
        assert_eq!(display, "abcdef1");
    }

    #[test]
    fn test_commit_hash_equality() {
        let a = CommitHash("abc".to_string());
        let b = CommitHash("abc".to_string());
        let c = CommitHash("def".to_string());
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
