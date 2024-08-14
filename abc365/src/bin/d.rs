use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut dp = vec![vec![0_usize; 3]; n + 1];
    for i in 0..n {
        for j in 0..3 {
            match s[i] {
                'R' => {
                    if j == 0 {
                        dp[i + 1][1] = dp[i + 1][1].max(dp[i][j] + 1);
                    } else if j == 1 {
                        dp[i + 1][0] = dp[i + 1][0].max(dp[i][j]);
                    } else {
                        dp[i + 1][0] = dp[i + 1][0].max(dp[i][j]);
                        dp[i + 1][1] = dp[i + 1][1].max(dp[i][j] + 1);
                    }
                }
                'P' => {
                    if j == 0 {
                        dp[i + 1][1] = dp[i + 1][1].max(dp[i][j]);
                        dp[i + 1][2] = dp[i + 1][2].max(dp[i][j] + 1);
                    } else if j == 1 {
                        dp[i + 1][2] = dp[i + 1][2].max(dp[i][j] + 1);
                    } else {
                        dp[i + 1][1] = dp[i + 1][1].max(dp[i][j]);
                    }
                }
                'S' => {
                    if j == 0 {
                        dp[i + 1][2] = dp[i + 1][2].max(dp[i][j]);
                    } else if j == 1 {
                        dp[i + 1][0] = dp[i + 1][0].max(dp[i][j] + 1);
                        dp[i + 1][2] = dp[i + 1][2].max(dp[i][j]);
                    } else {
                        dp[i + 1][0] = dp[i + 1][0].max(dp[i][j] + 1);
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    println!("{}", dp[n].iter().max().unwrap());
}
