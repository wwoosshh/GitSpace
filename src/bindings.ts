export interface RepoInfo { name: string; path: string; defaultBranch: string | null }
export interface Timeline { start: number; end: number }
export interface Author { id: string; name: string; email: string; color: string }
export interface Star {
  id: string; name: string; isDefault: boolean;
  authorId: string | null; headCommitId: string;
  position: [number, number, number]; color: string; bornAt: number;
}
export interface CommitNode {
  id: string; parents: string[]; branchId: string; authorId: string; timestamp: number;
  message: string; position: [number, number, number]; color: string; isMerge: boolean;
}
export interface MergeEvent {
  commitId: string; intoBranchId: string; fromBranchId: string;
  timestamp: number; position: [number, number, number];
}
export interface SceneMeta { totalCommits: number; sampled: boolean }
export interface SceneModel {
  repo: RepoInfo; timeline: Timeline; authors: Author[];
  branches: Star[]; commits: CommitNode[]; merges: MergeEvent[]; meta: SceneMeta;
}
