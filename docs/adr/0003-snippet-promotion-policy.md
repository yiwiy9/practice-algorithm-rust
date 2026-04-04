# ADR 0003: Snippet Promotion Policy

- Status: Accepted
- Date: 2026-04-04

## Context

最近の `E/F` では、方針だけでなく実装量も重く、問題ファイルごとに同じ helper を再発明している。
一方で、問題固有の観察をそのまま library 化すると `src/lib` が太りすぎる。

## Decision

`src/lib` に snippet を昇格させる条件を次のようにする。

1. 少なくとも 2 問以上で再利用価値が確認できる
2. 問題固有の意味づけを外しても API が自然
3. テスト可能な単位に切り出せる

思考パターンとして重要でも code にしにくいものは、`wiki/` に残す。

## Consequences

- 実装量の大きい最近の `E/F` に対して投資効果が出やすい
- library の肥大化を防げる
- code snippet と thought snippet の役割が分かれる
