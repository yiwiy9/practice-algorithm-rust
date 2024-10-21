use proconio::input;
const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        ab: [(usize,usize); n],
    }

    let sum = ab.iter().map(|(_, b)| b).sum::<usize>();

    let mut dp = vec![vec![vec![INF; 501]; 501]; n + 1];

    dp[0][0][0] = 0;
    for i in 0..n {
        for j in 0..501 {
            for k in 0..501 {
                if dp[i][j][k] == INF {
                    continue;
                }

                let (a, b) = ab[i];
                match a {
                    1 => {
                        if j + b < 501 {
                            dp[i + 1][j + b][k] = dp[i + 1][j + b][k].min(dp[i][j][k]);
                        }
                        if k + b < 501 {
                            dp[i + 1][j][k + b] = dp[i + 1][j][k + b].min(dp[i][j][k] + 1);
                        }
                        dp[i + 1][j][k] = dp[i + 1][j][k].min(dp[i][j][k] + 1);
                    }
                    2 => {
                        if j + b < 501 {
                            dp[i + 1][j + b][k] = dp[i + 1][j + b][k].min(dp[i][j][k] + 1);
                        }
                        if k + b < 501 {
                            dp[i + 1][j][k + b] = dp[i + 1][j][k + b].min(dp[i][j][k]);
                        }
                        dp[i + 1][j][k] = dp[i + 1][j][k].min(dp[i][j][k] + 1);
                    }
                    3 => {
                        if j + b < 501 {
                            dp[i + 1][j + b][k] = dp[i + 1][j + b][k].min(dp[i][j][k] + 1);
                        }
                        if k + b < 501 {
                            dp[i + 1][j][k + b] = dp[i + 1][j][k + b].min(dp[i][j][k] + 1);
                        }
                        dp[i + 1][j][k] = dp[i + 1][j][k].min(dp[i][j][k]);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    println!(
        "{}",
        if sum % 3 == 0 && dp[n][sum / 3][sum / 3] != INF {
            dp[n][sum / 3][sum / 3] as i64
        } else {
            -1
        }
    );
}
