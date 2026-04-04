# Contest + Recent Kenchon Weighted Analysis (2026-04-04)

## この分析の意図

`kenchon/` は恒久的な主戦場ではないが、
今回はユーザーが「ここ最近のコンテスト外学習として実力をよく反映している」と明示した。

そのため、固定ルールにはせず、
今回の snapshot に限って補助データとして加えた。

ただし `kenchon/` は ABC だけではなく JOI / JOIG / JOISC を多く含む。
したがって、「全部を ABC の E/F 難度に変換する」分析は不適切であり、
今回は次の 2 軸で扱う。

1. contest performance signal
2. broader training capability signal

## 対象と重み

### 主データ

- 最近 25 件の記録済み ABC
- 対象は `abc422` 〜 `abc451`
- 重みは `1.0`

### 補助データ

- `kenchon/` のうち、最近 90 日で更新された ABC 問題
- 39 問中、最近 90 日に入る ABC は 39 問
- うち `E/F/G` は 25 問
- 重みは `0.6`

### 補助データを 0.6 にした理由

- 実コンテストの方がレートへの直結度は高い
- 一方で `kenchon/` は最近の学習実態を表すので無視しすぎたくない
- そのため、主データより弱いが無視もしない重みとして `0.6` を採用した

### 量的に混ぜていないもの

- `kenchon/` 内の JOI / JOIG / JOISC 系

これらは難度や要求能力が ABC の文字難易度と直接対応しないため、
件数で混ぜず、弱点テーマの質的補強にだけ使った。

## 最近の `kenchon/` の構成

### 直近 30 日

- JOI family: 20
- ABC: 2
- その他: 1

### 直近 90 日

- ABC: 39
- JOI family: 35
- Typical90 / Tessoku / ARC など: 15

このため、`kenchon/` 全体の意味は
「最近の ABC 補助」ではなく
「総合トレーニングの主戦場」
と捉えるべきである。

## 数字

### 1. 最近 25 件の実コンテスト ABC

| 問題 | 自力 | コメントあり | 自力率 |
| --- | ---: | ---: | ---: |
| D | 16 / 24 | 8 / 24 | 67% |
| E | 6 / 23 | 17 / 23 | 26% |
| F | 2 / 7 | 5 / 7 | 29% |

### 2. 最近 90 日の `kenchon/` ABC

| 問題 | 自力 | コメントあり | 自力率 |
| --- | ---: | ---: | ---: |
| D | 5 / 10 | 5 / 10 | 50% |
| E | 6 / 11 | 5 / 11 | 55% |
| F | 8 / 13 | 5 / 13 | 62% |

### 3. 重み付き blended snapshot

`recent contest + 0.6 * recent kenchon`

| 問題 | 重み付き自力率 | 読み方 |
| --- | ---: | --- |
| D | 63% | 引き続き安定帯 |
| E | 32% | 依然として主ボトルネック |
| F | 46% | 実コンテスト記録だけよりずっと強い |

## この数字が意味すること

### 1. `E` はやはり今の主戦場

`kenchon/` を足しても `E` は一番弱い。
したがって、以前の結論は変わらない。
今いちばん伸ばすべきなのは `E` の contest conversion である。

### 2. `F` は「無理」ではない

実コンテストだけ見ると `F` はかなり苦しいが、
最近の `kenchon/` では `13` 問中 `8` 問がコメントなしで進んでいる。

つまり、`F` の能力そのものが欠落しているというより、
コンテスト本番でその能力を引き出せていない。

### 3. 一番大きい差は raw ability ではなく contest conversion gap

- 実コンテスト `E` 自力率: 26%
- `kenchon` `E` 自力率: 55%
- 実コンテスト `F` 自力率: 29%
- `kenchon` `F` 自力率: 62%

この差はかなり大きい。
したがって、今の停滞を
「最近の難問アルゴリズムが全然分からない」
と解釈するのは少しズレている。

むしろ本質は、

- 最初の 10 分から 20 分で何を固定するか
- どう状態を置くか
- online に見える問題を offline にできるか
- 既知の実装骨格を思い出して当てはめられるか

の contest conversion にある。

## JOI 系が追加する重いシグナル

JOI 系は、最近のあなたに必要な

- 複数アルゴリズムの組み合わせ
- 問題の reduction
- 状態設計
- 実装骨格の選択

を強く要求する。

そのため、ここは ABC `E/F` の補助問題ではなく、
`E/F` を解けるようにするための基礎体力トレーニングとして重く評価すべきである。

### 最近の JOI 系で見えている成長テーマ

#### 1. state design

- [`joi2015ho_b.rs`](../../kenchon/src/bin/joi2015ho_b.rs)
- [`joi2017yo_d.rs`](../../kenchon/src/bin/joi2017yo_d.rs)
- [`joigsp2024_a.rs`](../../kenchon/src/bin/joigsp2024_a.rs)

ここでは「DP を知っているか」より
「その DP 定義で後ろの状態に依存しないか」を問われている。

#### 2. reduction and viewpoint shift

- [`joi2020ho_b.rs`](../../kenchon/src/bin/joi2020ho_b.rs)
- [`abc254_f.rs`](../../kenchon/src/bin/abc254_f.rs)
- [`joi2024ho_b.rs`](../../kenchon/src/bin/joi2024ho_b.rs)

ここでは「そのまま解く」より
「固定するものを決めて、別問題へ落とす」力が鍛えられている。

#### 3. shortest path / graph state expansion

- [`joi2016yo_e.rs`](../../kenchon/src/bin/joi2016yo_e.rs)
- [`joi2017yo_f.rs`](../../kenchon/src/bin/joi2017yo_f.rs)
- [`joi2021_yo2_b.rs`](../../kenchon/src/bin/joi2021_yo2_b.rs)

ここでは BFS / Dijkstra 自体より、
危険状態や補助状態をどう持つかが主眼になっている。

#### 4. data structure as a finishing move, not a starting point

- [`joi2019ho_b.rs`](../../kenchon/src/bin/joi2019ho_b.rs)
- [`joi2023ho_a.rs`](../../kenchon/src/bin/joi2023ho_a.rs)
- [`joig2022_e.rs`](../../kenchon/src/bin/joig2022_e.rs)

ここでは、セグ木や lazy segtree を先に振り回すのではなく、
必要な reduction が見えた後に使う姿勢が重要になる。

## `kenchon/` の ABC 部分が追加した示唆

### 1. 実装力は contest-only 分析より強い

最近の `kenchon` `F` には、Fenwick Tree, Segtree, 行列的な遷移合成, BIT/累積/変換系が複数ある。

- [`abc432_e.rs`](../../kenchon/src/bin/abc432_e.rs)
- [`abc436_f.rs`](../../kenchon/src/bin/abc436_f.rs)
- [`abc437_f.rs`](../../kenchon/src/bin/abc437_f.rs)
- [`abc429_f.rs`](../../kenchon/src/bin/abc429_f.rs)
- [`abc441_e.rs`](../../kenchon/src/bin/abc441_e.rs)

ここから言えるのは、
「データ構造を実装できない」が主因ではないということ。
主因は、その reduction に時間内で辿り着く部分にある。

### 2. DP の state design が今の学習 frontier

最近の JOI 系では、状態定義そのものに言及したメモが多い。

- [`joi2015ho_b.rs`](../../kenchon/src/bin/joi2015ho_b.rs): 区間 DP の手番整理
- [`joi2017yo_d.rs`](../../kenchon/src/bin/joi2017yo_d.rs): 順序空間を bit DP へ圧縮
- [`joigsp2024_a.rs`](../../kenchon/src/bin/joigsp2024_a.rs): DP 定義が下手だと遷移が壊れる

これは、今の外練が
「アルゴリズムの名前」より「状態の切り方」
に寄っていることを示している。

### 3. 視点固定は引き続き重要

`kenchon/` 側でも、固定や視点変換のメモが効いている。

- [`abc254_f.rs`](../../kenchon/src/bin/abc254_f.rs): 代表値 + 差分に変換
- [`abc441_e.rs`](../../kenchon/src/bin/abc441_e.rs): 区間の右端を固定すると実装が楽
- [`joi2020ho_b.rs`](../../kenchon/src/bin/joi2020ho_b.rs): 尺取りを一段抽象化して左端固定 + 二分探索

このため、「何を固定するか」は contest だけでなく外練でも継続課題である。

## 更新後の評価

- コンテスト floor: `A-D`
- コンテストの主ボトルネック: `E`
- 総合育成の主戦場: `kenchon` の JOI 系を含む複合トレーニング
- 現在の off-contest ceiling: 強めの `E` から `F` 相当の思考まで含む
- 主問題: アルゴリズム知識不足より、contest conversion と初手の state design

## 今後の練習で重視すること

1. 実コンテスト `E` を解くとき、最初の 15 分で「固定するもの」「状態」「online/offline」を必ず紙に書く
2. `kenchon` の JOI/ABC を問わず、解けた理由を「知識」ではなく「初手の観点」「reduction」「state design」で記録する
3. snippet は実装高速化のために作るが、最優先は thought snippet 化
4. JOI 系で出てきた発想を、実コンテスト `E/F` にどう移植するかを明示的に書く

## 再生成コマンド

```bash
python3 src/contest/docs/tools/analyze_abc_history.py --kenchon-days 90 --kenchon-weight 0.6
```
