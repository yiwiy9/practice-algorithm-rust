use proconio::input;

const MOD: usize = 100000;

fn main() {
    input! {
        w: usize,
        h: usize,
    }

    let mut dp = vec![vec![vec![vec![0; 2]; 2]; w]; h];
    dp[0][1][0][0] = 1;
    dp[1][0][1][0] = 1;
    for i in 0..h {
        for j in 0..w {
            for dir in 0..2 {
                for not_turn in 0..2 {
                    if j + 1 < w {
                        if dir == 0 {
                            //西→東
                            dp[i][j + 1][0][0] += dp[i][j][dir][not_turn];
                            dp[i][j + 1][0][0] %= MOD;
                        } else if not_turn == 0 {
                            //北→東
                            dp[i][j + 1][0][1] += dp[i][j][dir][not_turn];
                            dp[i][j + 1][0][1] %= MOD;
                        }
                    }

                    if i + 1 < h {
                        if dir == 1 {
                            //北→南
                            dp[i + 1][j][1][0] += dp[i][j][dir][not_turn];
                            dp[i + 1][j][1][0] %= MOD;
                        } else if not_turn == 0 {
                            //西→南
                            dp[i + 1][j][1][1] += dp[i][j][dir][not_turn];
                            dp[i + 1][j][1][1] %= MOD;
                        }
                    }
                }
            }
        }
    }

    println!(
        "{}",
        dp[h - 1][w - 1].iter().fold(0, |acc, x| (acc
            + x.iter().fold(0, |acc, &x| (acc + x) % MOD))
            % MOD)
    );
}
