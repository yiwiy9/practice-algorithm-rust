use proconio::input;
const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let mut dp = vec![vec![0; m + 1]; n + 1];
    for j in 1..=m {
        dp[1][j] = 1;
    }

    for i in 1..n {
        let mut s = vec![0; m + 2];
        for j in 0..=m {
            s[j + 1] = (s[j] + dp[i][j]) % MOD;
        }

        for j in 1..=m {
            if k == 0 {
                dp[i + 1][j] = s[m + 1] - s[1];
                dp[i + 1][j] %= MOD;
                continue;
            }

            if j + k <= m {
                dp[i + 1][j] += MOD + s[m + 1] - s[j + k];
                dp[i + 1][j] %= MOD;
            }
            if j > k {
                dp[i + 1][j] += MOD + s[j - k + 1] - s[1];
                dp[i + 1][j] %= MOD;
            }
        }
    }

    let mut ans = 0;
    for j in 1..=m {
        ans += dp[n][j];
        ans %= MOD;
    }

    println!("{}", ans);
}
