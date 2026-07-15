# GitSpace 레이아웃 v2 — 시간 방향 DAG (설계 변경)

- **작성일:** 2026-07-15
- **배경:** V1의 "궤도 성좌(orbital)" 배치는 아름답지만 실사용 시 (a) 카메라 자동회전으로 정보를 못 읽고, (b) 시간의 흐름이 안 보이며, (c) 어떤 브랜치가 main에서 분기해 커밋이 쌓이고 머지되는지 이해 불가. 사용자 피드백으로 시각화 모델을 교체한다.

## 새 모델: 시간 방향 브랜치 흐름 (3D git-graph)

- **시간 = X축** (왼쪽 과거 → 오른쪽 미래). `x = (ts - start)/span * TIME_LENGTH`.
- **main = 중심 스파인** (lane 0, y=z=0). main 커밋이 X축을 따라 놓이고, main HEAD가 선두 별로 우측 끝(최신).
- **브랜치 = 시간축 둘레로 3D 팬**: 각 브랜치(lane i≥1)는 X축 주위 각도 θ_i(황금각 분포)와 반지름 `LANE_RADIUS`로 오프셋 → `(x, R·cosθ, R·sinθ)`. 브랜치 커밋은 그 lane 위에 시간순으로 놓임.
- **분기·머지는 엣지로 표현**: 각 커밋에서 부모 커밋으로 선을 그린다.
  - 브랜치 첫 커밋 → (main의) 분기 부모: 중심에서 lane으로 뻗는 대각선 = **분기(갈라짐)**.
  - 머지 커밋(main, 중심) → 2번째 부모(브랜치 tip): lane에서 중심으로 오는 대각선 = **머지(합류)**.
- **main 전진**: 스크러버 t가 커지면 `now` 시점 X가 우측으로 이동, main HEAD 선두 별이 우측으로 나아감.
- **카메라**: 자동회전 **OFF**. `OrbitControls`로 사용자가 드래그 회전·줌·팬 → **다각도 관찰**. 기본 시점은 스파인을 비스듬히 내려보는 3/4 각도.
- **우주 aesthetic 유지**: 스타필드·성운·Bloom 발광·작성자 색 그대로.

## 데이터 모델 변경

- `CommitNode`에 **`parents: Vec<String>`** 추가(= RawCommit.parents). 프론트가 커밋→부모 엣지를 그리는 데 필요. serde camelCase → TS `parents: string[]`.
- (부수효과) 프론트가 백엔드 좌표를 직접 잇게 되어 **궤도 공식 이중 정의(layout.rs ↔ OrbitLines.tsx)가 제거됨**.

## 좌표 매핑 (layout.rs)

```
TIME_LENGTH = 60.0, LANE_RADIUS = 8.0, GOLDEN = 2.39996 (rad)
x = if span > 0 { (ts - start) as f32 / span * TIME_LENGTH } else { 0.0 }
lane = branch_index(owning branch)   // main(default)=0, 나머지 이름순 1,2,...
position =
  if lane == 0 { [x, 0, 0] }
  else { let t = (lane-1) as f32 * GOLDEN; [x, LANE_RADIUS*cos(t), LANE_RADIUS*sin(t)] }
```

- 브랜치 별(Star) position = head 커밋 position(불변 규칙 유지). 머지 위치 = 머지 커밋 position.
- 단일 커밋(span=0)에서도 유한 좌표(x=0).

## 프론트 변경

- **삭제**: `OrbitLines.tsx`(궤도 원). 
- **추가**: `Edges.tsx` — 커밋 id→position 맵을 만들고, 각 커밋에서 각 부모로 선(drei `<Line>`, 약한 곡선/직선). 색은 자식 브랜치 색. 시간 필터된 커밋만.
- **SceneView**: 카메라 위치를 스파인 3/4 뷰로, `OrbitControls`에서 `autoRotate` 제거(수동 회전·줌·팬 유지). `<Edges commits={visible.commits}/>`로 교체. BranchStars/CommitNodes/MergeFlashes/Effects/Scrubber 유지.
- **selectVisible** 불변(시간 필터).

## 테스트 영향

- `layout.rs` 기존 테스트는 대부분 구조(레인 배정·머지·star==head·유한좌표·authors·timeline)를 검증 → 유지. 좌표 상수값을 직접 단언하는 테스트는 없음.
- 신규 테스트: (1) main 커밋은 y=z=0(중심), 브랜치 커밋은 오프셋(비영); (2) `CommitNode.parents`가 원시 부모와 일치.

## 범위

- V1 기능 유지(로컬 git, 예술 우선). 이 문서는 **레이아웃·카메라·엣지 렌더**만 교체한다. PR/CI/멀티저장소 등은 여전히 V2+ 밖.
