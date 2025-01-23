use proconio::input;

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        ab: [(usize,usize); n],
    }

    let b_sum = ab.iter().map(|&(_, b)| b).sum::<usize>();
    if b_sum % 3 != 0 {
        println!("-1");
        return;
    }

    let m = b_sum / 3;
    let mut dp = vec![vec![INF; m + 1]; m + 1];
    dp[0][0] = 0;
    for &(a, b) in &ab {
        let mut next_dp = vec![vec![INF; m + 1]; m + 1];
        for i in 0..=m {
            for j in 0..=m {
                if dp[i][j] == INF {
                    continue;
                }

                match a {
                    1 => {
                        if i + b <= m {
                            next_dp[i + b][j] = next_dp[i + b][j].min(dp[i][j]);
                        }
                        if j + b <= m {
                            next_dp[i][j + b] = next_dp[i][j + b].min(dp[i][j] + 1);
                        }
                        next_dp[i][j] = next_dp[i][j].min(dp[i][j] + 1);
                    }
                    2 => {
                        if i + b <= m {
                            next_dp[i + b][j] = next_dp[i + b][j].min(dp[i][j] + 1);
                        }
                        if j + b <= m {
                            next_dp[i][j + b] = next_dp[i][j + b].min(dp[i][j]);
                        }
                        next_dp[i][j] = next_dp[i][j].min(dp[i][j] + 1);
                    }
                    3 => {
                        if i + b <= m {
                            next_dp[i + b][j] = next_dp[i + b][j].min(dp[i][j] + 1);
                        }
                        if j + b <= m {
                            next_dp[i][j + b] = next_dp[i][j + b].min(dp[i][j] + 1);
                        }
                        next_dp[i][j] = next_dp[i][j].min(dp[i][j]);
                    }
                    _ => unreachable!(),
                }
            }
        }
        dp = next_dp;
    }

    println!(
        "{}",
        if dp[m][m] == INF {
            -1
        } else {
            dp[m][m] as isize
        }
    );
}
