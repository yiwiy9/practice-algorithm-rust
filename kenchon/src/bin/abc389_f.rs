use ac_library::{LazySegtree, MapMonoid, Max, Monoid};
use itertools::Itertools;
use proconio::input;

const MAX: usize = 500_010;

/**
 * 区間加算・区間最大値取得
 *
 * チートシート
 *  - https://betrue12.hateblo.jp/entry/2020/09/23/005940
 * 使い方の詳細
 *  - https://betrue12.hateblo.jp/entry/2020/09/22/194541
 */
#[derive(Clone)]
struct F;
impl MapMonoid for F {
    type M = Max<usize>;
    type F = usize;

    fn identity_map() -> Self::F {
        0
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        *f + *x
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        *f + *g
    }
}

/// https://atcoder.jp/contests/abc389/tasks/abc389_f
/// https://atcoder.jp/contests/abc389/editorial/11966
///
/// dp[i][j]=( 初めにレートが j であったときに i 回のコンテスト後のレート )
///
/// ポイント
/// まずこの dp 配列の特徴として、dp[i][j]≤dp[i][j+1] が必ず成り立ちます。レートは 1 ずつしか変動しないため、
/// 最初のレートが X のときは X+1 であった場合のレートと等しくなることはあっても
/// 等しくなったらそれ以降はレートは必ず等しくなりレートの初期値の大小とコンテスト後のレートの大小が入れ替わることはないです。
///
/// => 広義単調増加になるため、二分探索が使える
fn main() {
    input! {
        n: usize,
        lr: [(usize,usize); n],
        q: usize,
        x: [usize; q],
    }

    let mut lazy_segtree = LazySegtree::<F>::from((0..MAX).collect_vec());
    for &(l, r) in &lr {
        let left_x = lazy_segtree.max_right(0, |num| num < l);
        let right_x = lazy_segtree.max_right(0, |num| num <= r);
        lazy_segtree.apply_range(left_x..right_x, 1);
    }

    for &x_i in &x {
        println!("{}", lazy_segtree.get(x_i));
    }
}
