# ADR 0001: Documentation Structure

- Status: Accepted
- Date: 2026-04-04

## Context

AC コードの中に良い反省や解法メモはあるが、問題ファイルに分散している。
この状態だと、

- 似た詰まり方の再発に気づきにくい
- 今後の agent が継続して育てにくい
- 実力評価と練習方針が都度ぶれる

## Decision

`src/contest/docs` 配下に、次の役割分担でドキュメントを持つ。

- `wiki/`: 恒久知識
- `knowledge/`: 調査結果・分析スナップショット
- `adr/`: 方針決定の履歴
- `plans/`: 直近の行動
- `templates/`: 追記用の雛形
- `tools/`: 再分析用スクリプト

## Consequences

- コンテストコードの近くに知識が置かれるので更新しやすい
- 古いメモを恒久知識と一時メモに分けられる
- 今後の追加作業を人間・agent のどちらでも継続しやすい
