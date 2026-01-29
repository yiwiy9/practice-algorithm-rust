use num::integer::gcd;
use proconio::{input, marker::Usize1};
use std::collections::HashMap;

/// https://atcoder.jp/contests/abc442/tasks/abc442_e
/// https://atcoder.jp/contests/abc442/editorial/15136
/// => 偏角ソート の練習問題です。
fn main() {
    input! {
        n: usize,
        q: usize,
        xy: [(i64, i64); n],
        ab: [(Usize1,Usize1); q],
    }

    // (x,y) を方向キー (x/g, y/g) に正規化（同方向判定用）
    let dir_key = |p: (i64, i64)| -> (i64, i64) {
        let g = gcd(p.0, p.1);
        (p.0 / g, p.1 / g)
    };

    // 1) 各モンスターの方向キー
    let mut dirs_of = vec![(0i64, 0i64); n];
    for (i, &p) in xy.iter().enumerate() {
        dirs_of[i] = dir_key(p);
    }

    // 2) 方向ごとの個数を数える（同方向は同時に全滅するため）
    let mut cnt_map: HashMap<(i64, i64), i64> = HashMap::new();
    for &d in &dirs_of {
        *cnt_map.entry(d).or_insert(0) += 1;
    }

    // 3) ユニーク方向を偏角ソート（反時計回り）
    let mut unique_dirs: Vec<(i64, i64)> = cnt_map.keys().copied().collect();
    unique_dirs.sort_by(|a, b| cmp_by_argument_default_i64(*a, *b));

    let m = unique_dirs.len();

    // 4) 方向キー -> 偏角順 index
    let mut pos_map: HashMap<(i64, i64), usize> = HashMap::with_capacity(m * 2);
    for (i, &d) in unique_dirs.iter().enumerate() {
        pos_map.insert(d, i);
    }

    // 5) 偏角順のカウント配列 & 累積和
    let mut cnt = vec![0i64; m];
    for (&d, &c) in &cnt_map {
        let p = pos_map[&d];
        cnt[p] = c;
    }

    let mut ps = vec![0i64; m + 1];
    for i in 0..m {
        ps[i + 1] = ps[i] + cnt[i];
    }

    // 6) クエリ処理
    // 時計回り A -> B の総数
    // = 反時計回り B -> A の区間和（円環）に置換すると簡単
    for &(a, b) in &ab {
        let pa = pos_map[&dirs_of[a]];
        let pb = pos_map[&dirs_of[b]];

        let ans = if pa == pb {
            cnt[pa]
        } else if pb <= pa {
            // [pb, pa]
            ps[pa + 1] - ps[pb]
        } else {
            // wrap: [pb, m-1] + [0, pa]
            (ps[m] - ps[pb]) + (ps[pa + 1] - ps[0])
        };

        println!("{}", ans);
    }
}

use std::cmp::Ordering;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OriginPolicy {
    /// (0,0) を最初に置く（「原点は角度0扱い」みたいにしたい時）
    First,
    /// (0,0) を最後に置く（「とりあえず壊れない」デフォルト向け）
    Last,
    /// (0,0) が来たら panic（原点が出ないはずの問題でバグ検知したい時）
    Forbid,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TieBreak {
    /// 同偏角（同一直線）なら原点から近い順（norm^2 昇順）
    /// - 全順序（total order）にしやすく、安定で無難
    Norm2Asc,
    /// 同偏角なら (x,y) の辞書順
    /// - “方向”の比較というより「出力の見た目」を揃えたい用途向け
    Lex,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArgSortOptions {
    pub origin: OriginPolicy,
    pub tie: TieBreak,
}
impl Default for ArgSortOptions {
    fn default() -> Self {
        Self {
            origin: OriginPolicy::Last,
            tie: TieBreak::Norm2Asc,
        }
    }
}
#[inline]
fn upper_half(x: i64, y: i64) -> bool {
    y > 0 || (y == 0 && x >= 0)
}
#[inline]
fn cross(ax: i64, ay: i64, bx: i64, by: i64) -> i128 {
    (ax as i128) * (by as i128) - (ay as i128) * (bx as i128)
}
#[inline]
fn norm2(x: i64, y: i64) -> i128 {
    (x as i128) * (x as i128) + (y as i128) * (y as i128)
}
/// 偏角ソート（argument sort）のための比較関数（i64座標）
/// - 基準: +x軸を 0、反時計回りに増加する順（[0, 2π)）
/// - 実装: `atan2` を使わず、半平面判定 + 外積（cross）だけで比較（誤差なし・高速）
/// - 同偏角は tie-break で全順序にする（`sort_by` で安全に使える）
///
/// 典型解説（AtCoder ABC442 Editorial）:
/// - https://atcoder.jp/contests/abc442/editorial/15136
///
/// 注意:
/// - (0,0) は偏角が定義しづらいので `OriginPolicy` で扱いを決める
pub fn cmp_by_argument_i64(a: (i64, i64), b: (i64, i64), opt: ArgSortOptions) -> Ordering {
    let (ax, ay) = a;
    let (bx, by) = b;
    let a0 = ax == 0 && ay == 0;
    let b0 = bx == 0 && by == 0;
    if a0 || b0 {
        match opt.origin {
            OriginPolicy::First => return b0.cmp(&a0),
            OriginPolicy::Last => return a0.cmp(&b0),
            OriginPolicy::Forbid => panic!("(0,0) is not allowed in argument sort"),
        }
    }
    let ha = upper_half(ax, ay);
    let hb = upper_half(bx, by);
    if ha != hb {
        return hb.cmp(&ha);
    }
    let cr = cross(ax, ay, bx, by);
    if cr != 0 {
        return if cr > 0 {
            Ordering::Less
        } else {
            Ordering::Greater
        };
    }
    match opt.tie {
        TieBreak::Norm2Asc => norm2(ax, ay).cmp(&norm2(bx, by)),
        TieBreak::Lex => (ax, ay).cmp(&(bx, by)),
    }
}
/// デフォルト設定（原点は最後、同偏角は距離昇順）での比較
pub fn cmp_by_argument_default_i64(a: (i64, i64), b: (i64, i64)) -> Ordering {
    cmp_by_argument_i64(a, b, ArgSortOptions::default())
}
/// 任意の点型 T を (i64,i64) に写して偏角ソートする（オプション指定）
pub fn sort_by_argument_i64<T>(
    pts: &mut [T],
    to_xy: impl Fn(&T) -> (i64, i64),
    opt: ArgSortOptions,
) {
    pts.sort_by(|p, q| cmp_by_argument_i64(to_xy(p), to_xy(q), opt));
}
/// デフォルト設定で偏角ソートする
pub fn sort_by_argument_default_i64<T>(pts: &mut [T], to_xy: impl Fn(&T) -> (i64, i64)) {
    sort_by_argument_i64(pts, to_xy, ArgSortOptions::default());
}
