# Contest Prediction Playbook

このファイルは、コンテスト前の問題予想を継続的に改善するための
stable playbook である。

- 毎回見る変数
- signal の重み
- 予想時の手順
- 外しやすいパターン

だけを置く。

個別コンテストの予想と反省は `predictions/logs/` に分離する。

## なぜ `knowledge/` ではなく `predictions/` か

この内容は、

- 恒久知識だけではない
- 単発の分析 snapshot でもない
- contest ごとに pre/post のログが増える

という性質を持つ。

そのため、

- stable な運用ルール: `playbook.md`
- contest-specific な記録: `logs/`

をまとめる専用置き場として `predictions/` を使う。

## 収集する変数

各 contest で最低限集める。

| 変数 | 意味 | デフォルト |
| --- | --- | --- |
| `contest_id` | 予想対象のコンテスト | `abcXXX` |
| `contest_date` | 開催日 | 実日付 |
| `writers` | 告知に載っている writer | 公式告知から取得 |
| `score_band` | `D/E/F/G` の配点 | 公式告知から取得 |
| `recent_window` | 直近 ABC の参照範囲 | 直近 `6` 回 |
| `writer_window` | same-writer の公式解説担当回 | 可能な限り全回 |
| `negative_recency` | 直近 `1-2` 回と似すぎる型を下げる補正 | 有効 |
| `output_constructive_bias` | 出力構成問題の出やすさ | writer と配点で補正 |
| `global_invariant_bias` | parity / 色数 / 不変量型の出やすさ | writer と過去問で補正 |

## 現在の重み

現在の初期値は次で置く。
最後の calibration は `ABC454` 反映後。

| signal | weight | メモ |
| --- | ---: | --- |
| `writer_specific_signal` | `0.60` | 最重要。公式解説担当を優先する |
| `score_band_signal` | `0.20` | `425/450/525` の帯ごとの差 |
| `recent_trend_signal` | `0.10` | 直近 ABC の型の偏り |
| `negative_recency_signal` | `0.10` | 直近 `1-2` 回とかぶる型を下げる |

## 予想時の手順

1. 公式告知から `writers` と `score_band` を取る。
2. 直近 `6` 回の ABC の D/E/F をざっくり分類する。
3. same-writer の過去回を、**公式解説担当ベース** で拾う。
4. writer ごとに、よく出る軸を列挙する。
   - observation
   - constructive
   - graph / tree
   - parity / coloring / invariant
   - process compression
   - offline / data structure
5. `recent_trend_signal` は補助にとどめ、writer signal を上書きしない。
6. 予想は
   - 本命
   - 次点
   - 薄め
   の 3 段で書く。

## 予想時の禁止事項

- `直近で少なかったから出るはず` を主因にしない。
- 共作回では、**誰がどの問題の公式解説を書いたか** を見ずに writer 特性を一般化しない。
- `構成問題` を `状態圧縮` と雑にまとめない。
- グリッド / path / Hamiltonian 系で、二部グラフの色数条件を見落とさない。

## 現時点の補正メモ

### `sounansya`

- `output_constructive_bias` を高めに置く。
- `global_invariant_bias` を想像以上に高く置く。
- `process compression` に寄せすぎない。
- 「短い impossibility proof + 構成」を本命候補に入れる。

### `Nyaan`

- observation-heavy な D/E を強めに見る。
- 文字列 / 数列 / 遷移の正規形や圧縮状態を疑う。
- 実装の前に、何が invariant かを言い切るタイプを上げる。

## ログ更新ルール

- contest 前は `predictions/logs/` に新しいファイルを切る。
- contest 後は同じファイルに `Actual`, `Hit / Miss`, `Calibration` を追記する。
- 2 回以上同じ外し方をしたら、`wiki/stuck-checklist.md` か `wiki/thinking-routine.md` へ昇格する。
