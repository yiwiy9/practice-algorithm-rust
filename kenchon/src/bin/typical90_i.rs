use proconio::input;
use std::f64::consts::PI;
use superslice::*;

fn main() {
    input! {
        n: usize,
        xy: [(i64,i64); n],
    }

    let mut ans: f64 = 0.0;
    for (i, &(origin_x, origin_y)) in xy.iter().enumerate() {
        let mut shifted_xy = vec![];
        for (j, &(x, y)) in xy.iter().enumerate() {
            if i == j {
                continue;
            }
            shifted_xy.push((x - origin_x, y - origin_y));
        }

        shifted_xy.sort_by(|a, b| cmp_by_argument_default_i64(*a, *b));

        let m = shifted_xy.len();
        for idx in 0..m {
            let a = shifted_xy[idx];
            let target = (-a.0, -a.1); // 180度回転

            let pos = shifted_xy.lower_bound_by(|v| cmp_by_argument_default_i64(*v, target));

            for &t in &[pos % m, (pos + m - 1) % m] {
                let b = shifted_xy[t];
                let theta = angle_between(a, b); // [0, PI]
                ans = ans.max(theta);
            }
        }
    }

    println!("{:.10}", ans * 180.0 / PI);
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
/// 典型解説（AtCoder ABC442 Editorial）:
/// - https://atcoder.jp/contests/abc442/editorial/15136
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

#[inline]
fn dot(ax: i64, ay: i64, bx: i64, by: i64) -> i128 {
    (ax as i128) * (bx as i128) + (ay as i128) * (by as i128)
}
/// 2ベクトル a, b のなす角（0..=PI, ラジアン）を返す。
/// - 数値的に安定な atan2(|cross|, dot) を使う（acos より安全）
/// - a or b が (0,0) の場合は角度を 0 とみなす（この問題では通常出ないが安全側）
/// # Examples
/// - same direction: 0
/// - right angle: PI/2
/// - opposite direction: PI
pub fn angle_between(a: (i64, i64), b: (i64, i64)) -> f64 {
    let (ax, ay) = a;
    let (bx, by) = b;
    if (ax == 0 && ay == 0) || (bx == 0 && by == 0) {
        return 0.0;
    }
    let cr = cross(ax, ay, bx, by).abs() as f64;
    let dt = dot(ax, ay, bx, by) as f64;
    cr.atan2(dt)
}
