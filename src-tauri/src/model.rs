use serde::Serialize;

/// git_reader가 저장소에서 뽑아내는 원시 데이터 (IO 결과)
#[derive(Clone, Debug, PartialEq)]
pub struct RawCommit {
    pub id: String,
    pub parents: Vec<String>,
    pub author_name: String,
    pub author_email: String,
    pub timestamp: i64, // committer time, unix seconds
    pub message: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RawBranch {
    pub name: String,
    pub head: String,     // commit id
    pub is_default: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RepoData {
    pub name: String,
    pub path: String,
    pub head: Option<String>,
    pub branches: Vec<RawBranch>,
    /// newest-first (topo + time) 순회 결과
    pub commits: Vec<RawCommit>,
    pub sampled: bool,
}

/// 프론트로 넘어가는 씬 모델 (camelCase 직렬화)
#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SceneModel {
    pub repo: RepoInfo,
    pub timeline: Timeline,
    pub authors: Vec<Author>,
    pub branches: Vec<Star>,
    pub commits: Vec<CommitNode>,
    pub merges: Vec<MergeEvent>,
    pub meta: SceneMeta,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RepoInfo {
    pub name: String,
    pub path: String,
    pub default_branch: Option<String>,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Timeline {
    pub start: i64,
    pub end: i64,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub id: String,   // email 기준
    pub name: String,
    pub email: String,
    pub color: String, // hex
}

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Star {
    pub id: String,        // branch name
    pub name: String,
    pub is_default: bool,
    pub author_id: Option<String>,
    pub head_commit_id: String,
    pub position: [f32; 3],
    pub color: String,
    pub born_at: i64,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CommitNode {
    pub id: String,
    pub parents: Vec<String>,
    pub branch_id: String,
    pub author_id: String,
    pub timestamp: i64,
    pub message: String,
    pub position: [f32; 3],
    pub color: String,
    pub is_merge: bool,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MergeEvent {
    pub commit_id: String,
    pub into_branch_id: String,
    pub from_branch_id: String,
    pub timestamp: i64,
    pub position: [f32; 3],
}

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SceneMeta {
    pub total_commits: usize,
    pub sampled: bool,
}
