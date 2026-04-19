# AtCoder Growth Docs

このディレクトリは、`src/contest` に蓄積された AC コードをもとに、
今後の AtCoder 成長を継続的に管理するためのドキュメント基盤です。

## 目的

- 今の実力を、古い成功体験ではなく最近の ABC に寄せて評価する
- 「アルゴリズム名」ではなく、詰まり方と発想変換の弱点を残す
- 実コンテスト成績と、コンテスト外で伸びている総合力を分けて扱う
- 再利用できる実装を snippet 候補として切り出す
- 今後の方針変更や判断基準を ADR として残す

## 現在地（analysis snapshot: 2026-04-04 / repo state: 2026-04-06）

- 記録済みの分析 snapshot は `abc284` から `abc451` までの 115 コンテスト分
- リポジトリ自体には `abc452` が追加済みで、数値分析への反映は次回更新で行う
- 最近 25 件の記録済み ABC（`abc422` 〜 `abc451`）では、
  `D` はかなり戦えている一方で、`E` は「到達はするが自力化が弱い」状態
- コメント付きコードを「解説依存の近似 proxy」とみなすと、
  最近 25 件の `E` は `23` 本中 `17` 本、`F` は `7` 本中 `5` 本がコメント付き
- 一方で、最近の `kenchon/` は JOI 系を多く含む総合トレーニング群であり、
  実コンテスト結果とは別軸の「育成シグナル」として扱う

## ディレクトリ構成

- [`wiki/`](./wiki): 何度でも見返す恒久知識
- [`knowledge/`](./knowledge): 調査結果や分析スナップショット
- [`predictions/`](./predictions): 問題予想の運用ルールと contest ごとの予想ログ
- [`adr/`](./adr): 方針・運用ルール・判断基準の履歴
- [`plans/`](./plans): 直近の行動計画とバックログ
- [`templates/`](./templates): 今後メモを増やすときの雛形
- [`tools/`](./tools): 分析を再実行するための補助スクリプト

## まず見る場所

- 実力の現在地: [`wiki/current-skill-map.md`](./wiki/current-skill-map.md)
- 練習方法の基準: [`wiki/practice-strategy.md`](./wiki/practice-strategy.md)
- 詰まった時の戻り先: [`wiki/stuck-checklist.md`](./wiki/stuck-checklist.md)
- 問題予想の運用ルール: [`predictions/playbook.md`](./predictions/playbook.md)
- 直近の問題予想ログ: [`predictions/logs/2026-04-18-abc454.md`](./predictions/logs/2026-04-18-abc454.md)
- 直近の調査結果: [`knowledge/2026-04-04-contest-plus-kenchon-weighted-analysis.md`](./knowledge/2026-04-04-contest-plus-kenchon-weighted-analysis.md)
- 今やること: [`plans/current-plan.md`](./plans/current-plan.md)

## 更新ルール

1. コンテストや upsolve のあと、まず `plans/current-plan.md` の進捗を見る
2. 同じ詰まり方を 2 回以上したら `knowledge/` に追記する
3. 安定して再利用したい実装が見えたら `wiki/snippet-roadmap.md` を更新する
4. 「どう評価するか」「何を優先するか」が変わったら `adr/` に残す
5. 数字を更新したくなったら `python3 src/contest/docs/tools/analyze_abc_history.py` を実行する
6. その時点の学習主戦場ディレクトリを補助線として加えたいなら、ABC のように比較可能な部分は数値で、JOI のような異種問題群は質的分析で扱う
