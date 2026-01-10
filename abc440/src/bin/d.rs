use proconio::input;
use superslice::*;

/// https://atcoder.jp/contests/abc440/tasks/abc440_d
/// https://atcoder.jp/contests/abc440/editorial/15030
///
/// 前処理:
/// - A を昇順ソートして A[0..N-1]
/// - 各クエリ (X, Y) を処理
///
/// クエリ (X, Y):
/// 1) s = lower_bound(A, X)
///    - A[s] が「X以上で最小のA中の値」（無ければ s=N）
///
/// 2) t を動かしたとき、
///    「X以上 かつ A[t]以下 の整数の個数」 = A[t] - X + 1
///    「そのうち A に含まれる個数（区間 A[s..t]）」 = (t - s + 1)
///    よって
///    f(t) = (A[t] - X + 1) - (t - s + 1)
///         = 「X以上 かつ A[t]以下 で Aに含まれない数の個数」
///    ※ f(t) は t に対して単調増加
///
/// 3) 二分探索で最小の t_r を探す:
///    f(t_r) >= Y となる最小 t_r
///
/// 4) 答え:
///    ans = X + (Y - 1) + (t_r - s)
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i64; n],
        xy: [(i64,i64); q],
    }
    a.sort();

    for &(x, y) in &xy {
        let lower_i: i64 = a.lower_bound(&x) as i64;

        let mut left = lower_i - 1;
        let mut right = n as i64;
        while right - left > 1 {
            let mid = (left + right) / 2;

            if (a[mid as usize] - x + 1) - (mid - lower_i + 1) >= y {
                right = mid;
            } else {
                left = mid;
            }
        }

        println!("{}", x + (y - 1) + (right - lower_i));
    }
}
