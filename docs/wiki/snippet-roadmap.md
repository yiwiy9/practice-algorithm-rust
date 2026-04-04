# Snippet Roadmap

## 既にある強い手札（`src/lib`）

- BFS / DFS / Dijkstra / 01-BFS
- Doubling
- Rolling Hash
- Manacher
- 偏角ソート
- `sum_quotients`
- `tropical_matrix`（min-plus 行列）
- 素数・平方根・組合せなどの基本ライブラリ

つまり、基礎ライブラリは弱くありません。
足りていないのは、最近の `E/F` で何度も出てくる「実戦寄りの型」です。

## 優先度 A

### 1. `multi_source_bfs_with_owner`

- 目的: 各頂点に対して「最短の始点」「2 番目に近い始点」まで持つ
- 効く問題: `abc429/e`
- 効く理由: 多始点 BFS をただ回すだけではなく、source id を一緒に持つ型を固定できる

### 2. `rerooting_max` か汎用 rerooting の骨格

- 目的: 木の全方位 DP を prefix/suffix で書く骨格を固定する
- 効く問題: `abc428/e`
- 効く理由: 木 DP は知っていても、再 root 化の実装が毎回重い

### 3. `doubling_with_aggregate`

- 目的: 遷移先だけでなく、和やコストなどの付随情報も倍化で持つ
- 効く問題: `abc438/e`, `abc445/c`
- 効く理由: 「位置のダブリング」はあるが「情報のダブリング」がまだテンプレ化されていない

### 4. `offline_sweep_queries`

- 目的: クエリ先読みして、添字順や上限順に 1 回なめる型を固定する
- 効く問題: `abc435/e`, `abc449/e`
- 効く理由: online で重く考えすぎる癖を減らせる

## 優先度 B

### 5. `scc_condensation` か ACL wrapper

- 目的: SCC 分解して DAG に縮約する流れを短く書く
- 効く問題: `trainings_abc_d_e/abc335_e`
- 効く理由: 「サイクルがあるのでそのまま DP しない」を実装に落としやすくする

### 6. `permutation_cycle_decomposition`

- 目的: permutation / functional graph のサイクル分解をすぐ書けるようにする
- 効く問題: `abc436/e` など
- 効く理由: 自力で解けているテーマでも、実装をさらに軽くできる

### 7. `compress_intervals_for_segtree`

- 目的: 座標圧縮した点ではなく、隣接点間の区間をノードに持つ型
- 効く問題: `abc435/e`
- 効く理由: 最近の `E` で詰まりやすい「圧縮した後に何を segment tree の 1 点に対応させるか」を固定化できる

## 優先度 C

### 8. `state_graph_grid`

- 目的: `(x, y, dir)` や「辺を頂点化する」系の補助関数群
- 効く問題: `abc431/e`, `abc394/e`
- 効く理由: 発想の難しさは残るが、実装量はかなり削れる

### 9. `contribution_counting_cheatsheet`

- 目的: 実装ではなく wiki として、寄与計算の頻出パターンを整理する
- 効く問題: `abc423/e`, `abc436/f`, `abc449/d`
- 効く理由: これは code snippet より思考 snippet の方が重要

## snippet にしない方がいいもの

- 期待値 DP の式変形
- 数学的な式変形そのもの
- 問題固有の観察が強すぎる巨大 helper

これらは code 化より、`wiki/` の checklist と `knowledge/` の失敗パターン整理の方が効きます。
