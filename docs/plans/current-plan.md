# Current Plan

更新日: 2026-04-04

## 目標

- 最近の ABC `E` を「解説を見れば分かる」から「自力で方針が立つ」に変える
- 実装の重さを減らすために、`src/lib` に追加すべき snippet を洗い出す
- 詰まり方の再発を防ぐために、problem note の更新習慣を作る

## この plan の使い方

- 直近の作業では、まずこの plan を見て、今どの弱点を優先するかを合わせる
- 新しい調査や復習をしたら、完了した項目を消すのではなく、必要なら次の具体タスクに置き換える
- 問題を 1 問見たあと、恒久知識なら `wiki`、日付付きの発見なら `knowledge` に反映する

## 直近でやること

- [ ] 最近のコメント付き `E` を優先して revisit する
- [ ] 最近の JOI 系 `kenchon` と実コンテスト `E/F` の差を、「初手の観点」「状態定義」「reduction」で比較する
- [ ] `kenchon` で伸びている総合力を、contest conversion の観点で wiki に落とす
- [ ] 各問題について「最初に固定すべきだったもの」を 1 行で書く
- [ ] snippet 候補を `wiki/snippet-roadmap.md` から 2 個選んで着手順を決める

## 優先問題リスト

- [`abc429/e.rs`](../../abc429/src/bin/e.rs): 多始点 BFS + source 管理
- [`abc430/c.rs`](../../abc430/src/bin/c.rs): 固定と数え上げ
- [`abc431/e.rs`](../../abc431/src/bin/e.rs): 状態拡張グラフ + 01-BFS
- [`abc435/e.rs`](../../abc435/src/bin/e.rs): offline + 座標圧縮
- [`abc438/e.rs`](../../abc438/src/bin/e.rs): aggregate 付きダブリング
- [`abc449/e.rs`](../../abc449/src/bin/e.rs): クエリ先読み
- [`abc450/e.rs`](../../abc450/src/bin/e.rs): 誤読検知と再帰構造
- [`abc451/e.rs`](../../abc451/src/bin/e.rs): 条件の木構造復元

## 進め方

1. 1 問につき 20 分から 35 分だけ自力で考える
2. 止まったら `wiki/stuck-checklist.md` を見る
3. それでも無理なら元コードと editorial を見る
4. 最後に `templates/problem-note-template.md` で 4 項目だけ残す

## この plan を更新する条件

- 最近の重点テーマが変わったとき
- snippet 候補の優先順位が変わったとき
- 直近 8 問程度の振り返りで、自力化のボトルネックが変わったとき

## 成功条件

- 最近 8 問の `E` で、少なくとも半分はコメントなしで解ける
- `multi_source_bfs_with_owner` と `doubling_with_aggregate` の 2 つについて、実装方針が固まる
- 同じ詰まり方を `knowledge/` に二重登録しなくなる
