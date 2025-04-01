use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        k: usize,
        lr: [(usize, usize); k],
    }

    let mut dp = vec![0; n + 1];
    dp[1] = 1;
    let mut imos = vec![0; n + 1];
    let mut sum = 0;
    for i in 1..=n {
        sum += imos[i];
        sum %= MOD;
        dp[i] += sum;
        dp[i] %= MOD;

        for &(l, r) in &lr {
            if i + l <= n {
                imos[i + l] += dp[i];
                imos[i + l] %= MOD;
            }
            if i + r < n {
                imos[i + r + 1] += MOD - dp[i];
                imos[i + r + 1] %= MOD;
            }
        }
    }

    println!("{}", dp[n]);
}
