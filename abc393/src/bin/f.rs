use itertools::Itertools;
use proconio::{input, marker::Usize1};
use superslice::*;

const INF: usize = 1 << 60;

/// https://atcoder.jp/contests/abc393/tasks/abc393_f
/// https://atcoder.jp/contests/abc393/editorial/12252
/// LIS（最長単調増加部分列）の長さを求める際に典型的に用いられる以下の動的計画法を考えます。
/// - i=1,2…,N の順に Ai を見てゆき、以下の DP テーブルを in-place に更新する。
///    - dp[j] (1≤j≤N) = （今までに見た要素の中から取り出せる長さ j の単調増加部分列における、部分列の末尾の要素の最小値。
///                        ただし、そのような部分列が存在しない場合は ∞）
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        rx: [(Usize1, usize); q],
    }

    let mut x_by_r = vec![vec![]; n];
    for (i, &(r, x)) in rx.iter().enumerate() {
        x_by_r[r].push((i, x));
    }

    let mut dp = vec![INF; n + 1];
    let mut ans = vec![0; q];
    for i in 0..n {
        let lower_i = dp.lower_bound(&a[i]);
        dp[lower_i] = a[i];
        for &(j, x) in &x_by_r[i] {
            ans[j] = dp.upper_bound(&x);
        }
    }

    println!("{}", ans.iter().join("\n"));
}
