use itertools::Itertools;
use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/abc445/tasks/abc445_c
/// https://atcoder.jp/contests/abc445/editorial/15909
/// => 制約をくまなく読め！
///
/// そして、制約を読めなかったとしても、ダブリングを改造して解け！
/// この問題で求めたいのは「各 i について、i から k 回遷移した到達点」を出力すること。
///
/// ただし k = 10^100 なので、
/// - 整数型に入らない（u128でも無理）
/// - ふつうの 2進ダブリングで必要な「k のビットを見る」がやりにくい
///
/// しかし k は「10 のべき」で固定されているので、
/// 2進ではなく 10進の lifting（広義のダブリング）に改造して解く。
/// => dp[i][j] := j から 10^i 回遷移したときの到達点（0-indexed）
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    // dp[i][j]: j から 10^i 回遷移した到達点
    let mut dp = vec![vec![0usize; n]; 101];

    // i=0 は 10^0 = 1 回遷移
    dp[0].copy_from_slice(&a);

    // 遷移
    for i in 1..101 {
        for j in 0..n {
            let mut x = j;
            for _ in 0..10 {
                // 10^i = 10^(i-1) * 10
                x = dp[i - 1][x];
            }
            dp[i][j] = x;
        }
    }

    println!("{}", dp[100].iter().map(|dp_j| dp_j + 1).join(" "));
}
