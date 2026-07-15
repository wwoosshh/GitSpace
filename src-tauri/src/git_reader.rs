use crate::model::{RawBranch, RawCommit, RepoData};

pub const COMMIT_CAP: usize = 5000;

use git2::{BranchType, Repository, Sort};
use std::collections::HashMap;

pub fn read_repo(path: &str) -> Result<RepoData, String> {
    let repo = Repository::open(path).map_err(|e| format!("git 저장소를 열 수 없습니다: {e}"))?;

    let name = std::path::Path::new(path)
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "repo".into());

    // 기본 브랜치 이름 추정 (HEAD가 가리키는 브랜치, unborn이면 None)
    let default_branch = repo
        .head()
        .ok()
        .and_then(|h| h.shorthand().ok().map(|s| s.to_string()));
    let head_oid = repo.head().ok().and_then(|h| h.target()).map(|o| o.to_string());

    // 로컬 브랜치 열거
    let mut branches: Vec<RawBranch> = Vec::new();
    if let Ok(iter) = repo.branches(Some(BranchType::Local)) {
        for entry in iter {
            let (branch, _) = entry.map_err(|e| e.to_string())?;
            let bname = branch
                .name()
                .map_err(|e| e.to_string())?
                .unwrap_or("?")
                .to_string();
            if let Some(oid) = branch.get().target() {
                branches.push(RawBranch {
                    name: bname.clone(),
                    head: oid.to_string(),
                    is_default: Some(&bname) == default_branch.as_ref(),
                });
            }
        }
    }

    // 커밋 워크: 모든 브랜치 head를 push, 진짜 topo + time 정렬
    let mut commits: Vec<RawCommit> = Vec::new();
    let mut sampled = false;
    if repo.head().is_ok() {
        let mut walk = repo.revwalk().map_err(|e| e.to_string())?;
        walk.set_sorting(Sort::TOPOLOGICAL | Sort::TIME).map_err(|e| e.to_string())?;
        for b in &branches {
            if let Ok(oid) = git2::Oid::from_str(&b.head) {
                let _ = walk.push(oid);
            }
        }
        // head도 안전하게 push
        let _ = walk.push_head();

        let mut seen: HashMap<String, ()> = HashMap::new();
        for oid_res in walk {
            let oid = oid_res.map_err(|e| e.to_string())?;
            let id = oid.to_string();
            if seen.insert(id.clone(), ()).is_some() {
                continue;
            }
            if commits.len() >= COMMIT_CAP {
                sampled = true;
                break;
            }
            let c = repo.find_commit(oid).map_err(|e| e.to_string())?;
            let author = c.author();
            commits.push(RawCommit {
                id,
                parents: c.parent_ids().map(|p| p.to_string()).collect(),
                author_name: author.name().unwrap_or("").to_string(),
                author_email: author.email().unwrap_or("").to_string(),
                timestamp: c.time().seconds(),
                message: c.message().unwrap_or("").to_string(),
            });
        }
    }

    Ok(RepoData {
        name,
        path: path.to_string(),
        head: head_oid,
        branches,
        commits,
        sampled,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::{Repository, Signature};
    use std::fs;
    use tempfile::TempDir;

    /// 결정적 시각의 커밋을 만드는 헬퍼. 반환한 TempDir는 살려둘 것(drop=삭제).
    fn commit_file(
        repo: &Repository,
        name: &str,
        content: &str,
        msg: &str,
        secs: i64,
        parents: &[git2::Oid],
    ) -> git2::Oid {
        let dir = repo.workdir().unwrap();
        fs::write(dir.join(name), content).unwrap();
        let mut index = repo.index().unwrap();
        index.add_path(std::path::Path::new(name)).unwrap();
        index.write().unwrap();
        let tree = repo.find_tree(index.write_tree().unwrap()).unwrap();
        let when = git2::Time::new(secs, 0);
        let sig = Signature::new("Dev", "dev@x.com", &when).unwrap();
        let parent_commits: Vec<git2::Commit> =
            parents.iter().map(|oid| repo.find_commit(*oid).unwrap()).collect();
        let parent_refs: Vec<&git2::Commit> = parent_commits.iter().collect();
        repo.commit(Some("HEAD"), &sig, &sig, msg, &tree, &parent_refs).unwrap()
    }

    fn linear_repo() -> (TempDir, String) {
        let dir = TempDir::new().unwrap();
        let repo = Repository::init(dir.path()).unwrap();
        let c1 = commit_file(&repo, "a.txt", "1", "first", 100, &[]);
        let _c2 = commit_file(&repo, "a.txt", "2", "second", 200, &[c1]);
        let path = dir.path().to_string_lossy().to_string();
        (dir, path)
    }

    #[test]
    fn reads_linear_history_newest_first() {
        let (_dir, path) = linear_repo();
        let data = read_repo(&path).unwrap();
        assert_eq!(data.commits.len(), 2);
        // newest-first
        assert_eq!(data.commits[0].message, "second");
        assert_eq!(data.commits[0].timestamp, 200);
        assert_eq!(data.commits[1].message, "first");
        // parent 연결
        assert_eq!(data.commits[0].parents.len(), 1);
        assert!(data.commits[1].parents.is_empty());
        // 작성자
        assert_eq!(data.commits[0].author_email, "dev@x.com");
    }

    #[test]
    fn detects_default_branch_and_head() {
        let (_dir, path) = linear_repo();
        let data = read_repo(&path).unwrap();
        assert!(data.branches.iter().any(|b| b.is_default));
        assert!(data.head.is_some());
    }

    #[test]
    fn non_repo_path_returns_err() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().to_string_lossy().to_string();
        assert!(read_repo(&path).is_err());
    }

    #[test]
    fn empty_repo_returns_ok_with_no_commits() {
        let dir = TempDir::new().unwrap();
        Repository::init(dir.path()).unwrap();
        let path = dir.path().to_string_lossy().to_string();
        let data = read_repo(&path).unwrap();
        assert_eq!(data.commits.len(), 0);
        assert!(data.head.is_none());
    }
}
