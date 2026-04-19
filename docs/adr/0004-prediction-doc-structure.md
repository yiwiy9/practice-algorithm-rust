# ADR 0004: Prediction Docs Structure

- Status: Accepted
- Date: 2026-04-19

## Context

問題予想のドキュメントは、

- stable な運用ルール
- contest ごとの予想ログ
- contest 後の retro

を同時に持つ。

これを `knowledge/` に 1 ファイルまたは 2 ファイルで置くと、

- 恒久知識と案件ログが混ざる
- どこへ追記すべきか分かりにくい
- 予想依頼のたびにファイルの責務がぶれる

という問題がある。

## Decision

`src/contest/docs/predictions/` を新設し、
問題予想関連はここへ分離する。

- `predictions/playbook.md`: 安定した運用ルールと現在の重み
- `predictions/logs/`: contest-specific な pre/post 記録

`knowledge/` は引き続き、

- 調査結果
- 分析 snapshot
- 単発の知見

を置く。

## Consequences

- 予想 system と analysis snapshot の責務が分かれる
- contest 前後でどこを更新するか明確になる
- 今後の prediction 依頼を連続して受けても運用しやすい
