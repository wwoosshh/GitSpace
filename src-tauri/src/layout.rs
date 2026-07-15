use crate::model::*;
use std::collections::HashMap;

/// 작성자 색 팔레트 (index 순환)
const PALETTE: [&str; 8] = [
    "#4a90ff", "#ff9a3c", "#ff5a6a", "#57d9a3",
    "#c07bff", "#ffd166", "#4ecdc4", "#f78fb3",
];

const BASE_RADIUS: f32 = 6.0;
const RADIUS_STEP: f32 = 4.0;
const INCLINATION_STEP: f32 = 0.18; // radians per lane
const TURNS: f32 = 1.5;
const TWO_PI: f32 = std::f32::consts::TAU;

/// commit id -> branch name. 기본 브랜치 먼저, 그다음 이름순으로 first-parent 워크하며
/// 아직 소유되지 않은 커밋을 그 브랜치에 배정.
pub fn assign_lanes(data: &RepoData) -> HashMap<String, String> {
    let by_id: HashMap<&str, &RawCommit> =
        data.commits.iter().map(|c| (c.id.as_str(), c)).collect();

    let mut branches = data.branches.clone();
    branches.sort_by(|a, b| b.is_default.cmp(&a.is_default).then(a.name.cmp(&b.name)));

    let mut owner: HashMap<String, String> = HashMap::new();
    for br in &branches {
        let mut cur = Some(br.head.clone());
        while let Some(id) = cur {
            if owner.contains_key(&id) {
                break; // 이미 앞선 브랜치가 소유 → 공통 조상
            }
            owner.insert(id.clone(), br.name.clone());
            cur = by_id
                .get(id.as_str())
                .and_then(|c| c.parents.first().cloned()); // first-parent 체인
        }
    }
    owner
}

pub fn build_scene(data: &RepoData) -> SceneModel {
    let authors = build_authors(&data.commits);
    let timeline = build_timeline(&data.commits);
    let author_color: HashMap<&str, &str> =
        authors.iter().map(|a| (a.email.as_str(), a.color.as_str())).collect();

    let lanes = assign_lanes(data);

    // 브랜치 인덱스(궤도 번호): 기본 먼저, 이름순
    let mut ordered_branches = data.branches.clone();
    ordered_branches
        .sort_by(|a, b| b.is_default.cmp(&a.is_default).then(a.name.cmp(&b.name)));
    let branch_index: HashMap<&str, usize> = ordered_branches
        .iter()
        .enumerate()
        .map(|(i, b)| (b.name.as_str(), i))
        .collect();

    let span = (timeline.end - timeline.start).max(1) as f32;

    // 커밋 노드
    let mut commit_nodes: Vec<CommitNode> = Vec::with_capacity(data.commits.len());
    let mut pos_by_id: HashMap<String, [f32; 3]> = HashMap::new();
    for c in &data.commits {
        let branch = lanes.get(&c.id).cloned().unwrap_or_else(|| {
            // 어떤 브랜치에도 안 걸린 커밋(예: orphan) → 기본 또는 첫 브랜치
            ordered_branches.first().map(|b| b.name.clone()).unwrap_or_else(|| "detached".into())
        });
        let idx = branch_index.get(branch.as_str()).copied().unwrap_or(0);
        let t01 = (c.timestamp - timeline.start) as f32 / span;
        let position = orbit_position(idx, t01);
        pos_by_id.insert(c.id.clone(), position);

        let color = author_color.get(c.author_email.as_str()).copied().unwrap_or("#ffffff");
        commit_nodes.push(CommitNode {
            id: c.id.clone(),
            branch_id: branch,
            author_id: c.author_email.clone(),
            timestamp: c.timestamp,
            message: c.message.clone(),
            position,
            color: color.to_string(),
            is_merge: c.parents.len() >= 2,
        });
    }

    // 브랜치 별 (head 커밋 위치)
    let mut stars: Vec<Star> = Vec::new();
    for b in &ordered_branches {
        let position = pos_by_id.get(&b.head).copied().unwrap_or([0.0, 0.0, 0.0]);
        let head_commit = data.commits.iter().find(|c| c.id == b.head);
        let author_email = head_commit.map(|c| c.author_email.clone());
        let color = author_email
            .as_deref()
            .and_then(|e| author_color.get(e).copied())
            .unwrap_or("#ffffff")
            .to_string();
        stars.push(Star {
            id: b.name.clone(),
            name: b.name.clone(),
            is_default: b.is_default,
            author_id: author_email,
            head_commit_id: b.head.clone(),
            position,
            color,
            born_at: head_commit.map(|c| c.timestamp).unwrap_or(timeline.start),
        });
    }

    SceneModel {
        repo: RepoInfo {
            name: data.name.clone(),
            path: data.path.clone(),
            default_branch: data.branches.iter().find(|b| b.is_default).map(|b| b.name.clone()),
        },
        timeline,
        authors,
        branches: stars,
        commits: commit_nodes,
        merges: Vec::new(),
        meta: SceneMeta {
            total_commits: data.commits.len(),
            sampled: data.sampled,
        },
    }
}

/// 궤도 위 위치: XZ 평면 타원을 lane 경사만큼 X축 회전
fn orbit_position(lane: usize, t01: f32) -> [f32; 3] {
    let radius = BASE_RADIUS + lane as f32 * RADIUS_STEP;
    let incl = lane as f32 * INCLINATION_STEP;
    let angle = t01 * TWO_PI * TURNS;
    let px = angle.cos() * radius;
    let pz = angle.sin() * radius;
    // X축 회전 (py = 0)
    let y = -pz * incl.sin();
    let z = pz * incl.cos();
    [px, y, z]
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

    #[test]
    fn authors_deduplicate_commits_from_same_email() {
        let data = repo(
            vec![
                commit("c2", "a@x.com", 200, &["c1"]),
                commit("c1", "a@x.com", 100, &[]),
            ],
            vec![RawBranch { name: "main".into(), head: "c2".into(), is_default: true }],
        );
        let scene = build_scene(&data);
        // 같은 email의 두 커밋 → 작성자 1명으로 유일화
        assert_eq!(scene.authors.len(), 1);
        assert_eq!(scene.authors[0].email, "a@x.com");
    }

    #[test]
    fn lanes_assign_commits_to_branches_by_first_parent() {
        // main: c1<-c2<-c4 (merge), feature: c1<-c3
        let data = repo(
            vec![
                commit("c4", "a@x.com", 400, &["c2", "c3"]), // merge
                commit("c3", "b@x.com", 300, &["c1"]),
                commit("c2", "a@x.com", 200, &["c1"]),
                commit("c1", "a@x.com", 100, &[]),
            ],
            vec![
                RawBranch { name: "main".into(), head: "c4".into(), is_default: true },
                RawBranch { name: "feature".into(), head: "c3".into(), is_default: false },
            ],
        );
        let lanes = assign_lanes(&data);
        assert_eq!(lanes["c4"], "main");
        assert_eq!(lanes["c2"], "main");
        assert_eq!(lanes["c1"], "main");   // 기본 브랜치가 먼저 소유
        assert_eq!(lanes["c3"], "feature");
    }

    #[test]
    fn commits_and_branch_stars_are_populated_with_positions() {
        let data = repo(
            vec![
                commit("c2", "a@x.com", 200, &["c1"]),
                commit("c1", "a@x.com", 100, &[]),
            ],
            vec![RawBranch { name: "main".into(), head: "c2".into(), is_default: true }],
        );
        let scene = build_scene(&data);
        assert_eq!(scene.commits.len(), 2);
        // 커밋은 자신의 브랜치 색을 가진다
        assert_eq!(scene.commits[0].branch_id, "main");
        // 브랜치 별 1개, head 커밋 위치와 동일
        assert_eq!(scene.branches.len(), 1);
        let head = scene.commits.iter().find(|c| c.id == "c2").unwrap();
        assert_eq!(scene.branches[0].position, head.position);
        assert_eq!(scene.branches[0].head_commit_id, "c2");
    }

    #[test]
    fn single_commit_timeline_does_not_divide_by_zero() {
        let data = repo(
            vec![commit("c1", "a@x.com", 100, &[])],
            vec![RawBranch { name: "main".into(), head: "c1".into(), is_default: true }],
        );
        let scene = build_scene(&data);
        // start==end 여도 위치가 유한값이어야 함
        for coord in scene.commits[0].position {
            assert!(coord.is_finite());
        }
    }
}
