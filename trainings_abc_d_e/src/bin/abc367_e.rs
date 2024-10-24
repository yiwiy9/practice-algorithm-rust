use itertools::Itertools;
use proconio::{input, marker::Usize1};

/// ダブリング
/// https://zenn.dev/fjnkt98/articles/3c0c21778b6101
fn main() {
    input! {
        n: usize,
        k: usize,
        x: [Usize1; n],
        a: [usize; n],
    }

    // dp[i][j]: j番目の要素から2^i回遷移したときの到達地点
    let mut dp = vec![vec![0; n]; 61];

    // 初期条件
    dp[0][..n].copy_from_slice(&x[..n]);

    // 遷移
    for i in 1..61 {
        for j in 0..n {
            // 2^1 = 2^(i-1) + 2^(i-1)
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }

    let mut ans = vec![0; n];
    for i in 0..n {
        let mut j = i;
        for bit in 0..61 {
            if k & (1 << bit) != 0 {
                j = dp[bit][j];
            }
        }
        ans[i] = a[j];
    }

    println!("{}", ans.iter().join(" "));
}
