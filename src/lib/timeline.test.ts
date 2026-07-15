import { describe, it, expect } from 'vitest';
import { selectVisible } from './timeline';
import type { SceneModel } from '../bindings';

const scene: SceneModel = {
  repo: { name: 'demo', path: '/demo', defaultBranch: 'main' },
  timeline: { start: 100, end: 300 },
  authors: [{ id: 'a@x.com', name: 'a', email: 'a@x.com', color: '#4a90ff' }],
  branches: [{
    id: 'main', name: 'main', isDefault: true, authorId: 'a@x.com',
    headCommitId: 'c3', position: [0, 0, 0], color: '#4a90ff', bornAt: 300,
  }],
  commits: [
    { id: 'c1', parents: [], branchId: 'main', authorId: 'a@x.com', timestamp: 100, message: 'a', position: [0,0,0], color: '#4a90ff', isMerge: false },
    { id: 'c2', parents: ['c1'], branchId: 'main', authorId: 'a@x.com', timestamp: 200, message: 'b', position: [0,0,0], color: '#4a90ff', isMerge: false },
    { id: 'c3', parents: ['c2'], branchId: 'main', authorId: 'a@x.com', timestamp: 300, message: 'c', position: [0,0,0], color: '#4a90ff', isMerge: false },
  ],
  merges: [],
  meta: { totalCommits: 3, sampled: false, axisLength: 5 },
};

describe('selectVisible', () => {
  it('t=1 shows all commits', () => {
    const v = selectVisible(scene, 1);
    expect(v.commits.length).toBe(3);
  });
  it('t=0 shows only the earliest commit', () => {
    const v = selectVisible(scene, 0);
    expect(v.commits.map((c) => c.id)).toEqual(['c1']);
  });
  it('midpoint shows commits up to that time (inclusive)', () => {
    // t=0.5 -> timestamp 200
    const v = selectVisible(scene, 0.5);
    expect(v.commits.map((c) => c.id)).toEqual(['c1', 'c2']);
  });
  it('only branches whose bornAt<=currentTime are visible', () => {
    const v = selectVisible(scene, 0.5); // time=200 < bornAt 300
    expect(v.branches.length).toBe(0);
  });
});
