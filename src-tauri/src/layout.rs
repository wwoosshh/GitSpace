use crate::model::*;

/// 작성자 색 팔레트 (index 순환)
const PALETTE: [&str; 8] = [
    "#4a90ff", "#ff9a3c", "#ff5a6a", "#57d9a3",
    "#c07bff", "#ffd166", "#4ecdc4", "#f78fb3",
];

pub fn build_scene(data: &RepoData) -> SceneModel {
    let authors = build_authors(&data.commits);
    let timeline = build_timeline(&data.commits);

    SceneModel {
        repo: RepoInfo {
            name: data.name.clone(),
            path: data.path.clone(),
            default_branch: data.branches.iter().find(|b| b.is_default).map(|b| b.name.clone()),
        },
        timeline,
        authors,
        branches: Vec::new(),
        commits: Vec::new(),
        merges: Vec::new(),
        meta: SceneMeta {
            total_commits: data.commits.len(),
            sampled: data.sampled,
        },
    }
}

fn build_timeline(commits: &[RawCommit]) -> Timeline {
    let mut start = i64::MAX;
    let mut end = i64::MIN;
    for c in commits {
        start = start.min(c.timestamp);
        end = end.max(c.timestamp);
    }
    if commits.is_empty() {
        Timeline { start: 0, end: 0 }
    } else {
        Timeline { start, end }
    }
}

/// 등장 순서(입력 순서)를 보존하며 email로 유일화, 팔레트 순환 색 배정
fn build_authors(commits: &[RawCommit]) -> Vec<Author> {
    let mut seen: Vec<Author> = Vec::new();
    for c in commits {
        if seen.iter().any(|a| a.email == c.author_email) {
            continue;
        }
        let color = PALETTE[seen.len() % PALETTE.len()].to_string();
        seen.push(Author {
            id: c.author_email.clone(),
            name: c.author_name.clone(),
            email: c.author_email.clone(),
            color,
        });
    }
    seen
}

#[cfg(test)]
mod tests {
    use super::*;

    fn commit(id: &str, email: &str, ts: i64, parents: &[&str]) -> RawCommit {
        RawCommit {
            id: id.into(),
            parents: parents.iter().map(|s| s.to_string()).collect(),
            author_name: email.split('@').next().unwrap().into(),
            author_email: email.into(),
            timestamp: ts,
            message: format!("commit {id}"),
        }
    }

    fn repo(commits: Vec<RawCommit>, branches: Vec<RawBranch>) -> RepoData {
        RepoData {
            name: "demo".into(),
            path: "/demo".into(),
            head: commits.first().map(|c| c.id.clone()),
            branches,
            commits,
            sampled: false,
        }
    }

    #[test]
    fn timeline_spans_min_and_max_timestamp() {
        let data = repo(
            vec![
                commit("c3", "a@x.com", 300, &["c2"]),
                commit("c2", "b@x.com", 200, &["c1"]),
                commit("c1", "a@x.com", 100, &[]),
            ],
            vec![RawBranch { name: "main".into(), head: "c3".into(), is_default: true }],
        );
        let scene = build_scene(&data);
        assert_eq!(scene.timeline.start, 100);
        assert_eq!(scene.timeline.end, 300);
    }

    #[test]
    fn authors_are_unique_by_email_with_distinct_colors() {
        let data = repo(
            vec![
                commit("c2", "b@x.com", 200, &["c1"]),
                commit("c1", "a@x.com", 100, &[]),
            ],
            vec![RawBranch { name: "main".into(), head: "c2".into(), is_default: true }],
        );
        let scene = build_scene(&data);
        assert_eq!(scene.authors.len(), 2);
        // 결정적 색 배정: 등장 순서(newest-first)대로 팔레트
        let by_email: std::collections::HashMap<_, _> =
            scene.authors.iter().map(|a| (a.email.clone(), a.color.clone())).collect();
        assert_eq!(by_email["b@x.com"], "#4a90ff");
        assert_eq!(by_email["a@x.com"], "#ff9a3c");
    }

    #[test]
    fn empty_repo_yields_zero_timeline() {
        let data = repo(vec![], vec![]);
        let scene = build_scene(&data);
        assert_eq!(scene.timeline.start, 0);
        assert_eq!(scene.timeline.end, 0);
        assert_eq!(scene.meta.total_commits, 0);
    }
}
