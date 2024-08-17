use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        x: [Usize1; n],
        a: [usize; n],
    }

    // ダブリングで本番ACできた
    // 参考: https://zenn.dev/fjnkt98/articles/3c0c21778b6101
    let mut dp = vec![vec![0; n]; 61];
    for j in 0..n {
        dp[0][j] = x[j];
    }

    for i in 1..61 {
        for j in 0..n {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }

    let mut ans = vec![];
    for j in 0..n {
        let mut ans_j = j;
        let mut k_j = k;
        let mut i = 0;
        while k_j > 0 {
            if k_j & 1 == 1 {
                ans_j = dp[i][ans_j];
            }
            k_j >>= 1;
            i += 1;
        }
        ans.push(ans_j);
    }

    println!("{}", ans.iter().map(|&j| a[j]).join(" "));
}
