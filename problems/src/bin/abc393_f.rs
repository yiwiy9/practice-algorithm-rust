use itertools::Itertools;
use proconio::{input, marker::Usize1};
use superslice::*;

const INF: usize = 1 << 60;

/// https://atcoder.jp/contests/abc393/tasks/abc393_f
/// https://atcoder.jp/contests/abc393/editorial/12252
/// 2回目の挑戦 失敗
/// クエリを先読みして、LIS（最長単調増加部分列）の長さを求める際にそのiに対する答えを常に更新しておく
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        rx: [(Usize1, usize); q],
    }

    let mut queries = vec![vec![]; n];
    for (j, (r, x)) in rx.into_iter().enumerate() {
        queries[r].push((x, j));
    }

    let mut dp = vec![INF; n];
    let mut ans = vec![0; q];
    for i in 0..n {
        let lower_index = dp.lower_bound(&a[i]); // 狭義単調増加なので lower_bound
        dp[lower_index] = a[i]; // 各断面の増加列ではなく、あくまで各断面のLISの長さがわかる
        for &(x, j) in &queries[i] {
            let upper_index = dp.upper_bound(&x);
            ans[j] = upper_index;
        }
    }

    println!("{}", ans.iter().join("\n"));
}
