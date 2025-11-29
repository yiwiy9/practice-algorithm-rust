use proconio::input;
use std::collections::BinaryHeap;

/// https://atcoder.jp/contests/abc407/tasks/abc407_e
/// https://atcoder.jp/contests/abc407/editorial/13106
///
/// 【TLE】
/// 括弧列→前から決められる→DP
/// dp[i][j] := i番目の要素まで括弧の向きを決めた状態で、j = 左-右 のときの答えの最大値
///
/// 【解説AC】
/// 括弧列→前から決められる→i番目の要素の時点で `(` の数は必ず i/2 以上でないといけない
/// 1. k = 0..n において、各ステップごとに `(` をどこにするか決めていく
///    候補の集合に 2*k, 2*k-1 番目の要素を追加する
///    候補の集合から MAX を一つ取り出し、`(`とする
/// 2. 残りの 2*n - n 個の要素をすべて `)` とする
///
/// 前から括弧列の正当性を担保できるという性質からこの貪欲が成り立つ。
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            a: [usize; 2*n],
        }

        let mut max_heap = BinaryHeap::new();
        let mut ans = a[0];
        for k in 1..n {
            max_heap.push(a[2 * k]);
            max_heap.push(a[2 * k - 1]);

            let score = max_heap.pop().unwrap();
            ans += score;
        }

        println!("{}", ans);
    }
}
