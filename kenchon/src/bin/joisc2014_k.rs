use proconio::input;

/// https://atcoder.jp/contests/joisc2014/tasks/joisc2014_k
/// https://www2.ioi-jp.org/camp/2014/2014-sp-tasks/2014-sp-d4.pdf
/// https://www2.ioi-jp.org/camp/2014/2014-sp-tasks/2014-sp-d4-constellation2-review.pdf
fn main() {
    input! {
        n: usize,
        xyc: [(i64,i64,usize); n],
    }

    // 復習:
    // - 固定するもの:
    //   2 つの三角形を分ける共通接線上の 2 頂点 (u, v) を固定する。
    //   この直線を固定すると、残り 4 頂点は直線の左右に 2 個ずつ分かれる。
    // - 欲しい量:
    //   直線 u -> v の左側 / 右側に、各色が何個あるか。
    //   直線上の頂点の色が c なら、その側で作れる良い三角形数は
    //   「c 以外の 2 色の個数の積」で求まる。
    // - 必要クエリ:
    //   固定した u に対し、相手 v を動かしたときに
    //   「u -> v の左側にある各色の個数」を高速に知りたい。
    // - 単調性 / 境界:
    //   u から見た他点を偏角順に並べると、cross(vec[j], vec[k]) > 0 を満たす点群は
    //   連続区間になる。j を進めると区間右端も単調にしか動かないので尺取りできる。
    let ans = solve(&xyc);
    println!("{}", ans);
}

#[derive(Debug, Clone, Copy)]
struct Ray {
    dx: i64,
    dy: i64,
    color: usize,
}

fn solve(xyc: &[(i64, i64, usize)]) -> i128 {
    let n = xyc.len();
    let mut total = [0_i32; 3];
    for &(_, _, c) in xyc {
        total[c] += 1;
    }

    // 各候補は「2 本の共通接線」×「各直線の 2 方向」で合計 4 回数えられる。
    let mut ans4 = 0_i128;

    for u in 0..n {
        let (ux, uy, uc) = xyc[u];
        let mut rays = Vec::with_capacity(n - 1);
        for v in 0..n {
            if u == v {
                continue;
            }
            let (vx, vy, vc) = xyc[v];
            rays.push(Ray {
                dx: vx - ux,
                dy: vy - uy,
                color: vc,
            });
        }

        sort_by_argument_default_i64(&mut rays, |ray| (ray.dx, ray.dy));

        let m = rays.len();
        let mut rays2 = Vec::with_capacity(2 * m);
        rays2.extend_from_slice(&rays);
        rays2.extend_from_slice(&rays);

        // duplicated array 上の連続区間の色数を O(1) で取るための累積和
        let mut prefix = vec![[0_i32; 3]; 2 * m + 1];
        for i in 0..(2 * m) {
            prefix[i + 1] = prefix[i];
            prefix[i + 1][rays2[i].color] += 1;
        }

        let mut r = 1_usize;
        for j in 0..m {
            if r < j + 1 {
                r = j + 1;
            }
            while r < j + m && cross(rays[j].dx, rays[j].dy, rays2[r].dx, rays2[r].dy) > 0 {
                r += 1;
            }

            // (j, r) が u -> rays[j] の左側にある点たち
            let left = range_count(&prefix, j + 1, r);

            // 全体 = 左側 + 右側 + 直線上の 2 点(u, v)
            let mut right = [0_i32; 3];
            for color in 0..3 {
                right[color] = total[color] - left[color];
            }
            right[uc] -= 1;
            right[rays[j].color] -= 1;

            // 左の三角形が u を使い、右の三角形が v を使う場合
            ans4 += ways(&left, uc) * ways(&right, rays[j].color);
            // 左の三角形が v を使い、右の三角形が u を使う場合
            ans4 += ways(&left, rays[j].color) * ways(&right, uc);
        }
    }

    ans4 / 4
}

fn range_count(prefix: &[[i32; 3]], l: usize, r: usize) -> [i32; 3] {
    let mut cnt = [0_i32; 3];
    for color in 0..3 {
        cnt[color] = prefix[r][color] - prefix[l][color];
    }
    cnt
}

fn ways(cnt: &[i32; 3], line_color: usize) -> i128 {
    let mut res = 1_i128;
    for color in 0..3 {
        if color != line_color {
            res *= cnt[color] as i128;
        }
    }
    res
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
