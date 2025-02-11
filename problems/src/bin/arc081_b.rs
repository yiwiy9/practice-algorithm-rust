use proconio::{input, marker::Chars};
const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        s1: Chars,
        s2: Chars,
    }

    let mut dp = vec![vec![0; 3]; 3];

    let mut i = 0;
    if s1[0] == s2[0] {
        dp[0][0] = 1;
        dp[1][1] = 1;
        dp[2][2] = 1;
        i = 1;
    } else {
        dp[0][1] = 1;
        dp[0][2] = 1;
        dp[1][0] = 1;
        dp[1][2] = 1;
        dp[2][0] = 1;
        dp[2][1] = 1;
        i = 2;
    }

    while i < n {
        let mut next_dp = vec![vec![0; 3]; 3];
        for j in 0..3 {
            for k in 0..3 {
                if s1[i] == s2[i] {
                    for d in 0..3 {
                        if d != j && d != k {
                            next_dp[d][d] += dp[j][k];
                            next_dp[d][d] %= MOD;
                        }
                    }
                } else {
                    for d1 in 0..3 {
                        for d2 in 0..3 {
                            if d1 != j && d2 != k && d1 != d2 {
                                next_dp[d1][d2] += dp[j][k];
                                next_dp[d1][d2] %= MOD;
                            }
                        }
                    }
                }
            }
        }
        dp = next_dp;
        if s1[i] == s2[i] {
            i += 1;
        } else {
            i += 2;
        }
    }

    let mut ans = 0;
    for i in 0..3 {
        for j in 0..3 {
            ans += dp[i][j];
            ans %= MOD;
        }
    }

    println!("{}", ans);
}
