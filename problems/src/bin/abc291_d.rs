use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        ab: [(usize,usize); n],
    }

    let mut dp = vec![vec![0; 2]; n];
    dp[0][0] = 1;
    dp[0][1] = 1;
    for i in 1..n {
        if ab[i].0 != ab[i - 1].0 {
            dp[i][0] += dp[i - 1][0];
            dp[i][0] %= MOD;
        }
        if ab[i].1 != ab[i - 1].0 {
            dp[i][1] += dp[i - 1][0];
            dp[i][1] %= MOD;
        }
        if ab[i].0 != ab[i - 1].1 {
            dp[i][0] += dp[i - 1][1];
            dp[i][0] %= MOD;
        }
        if ab[i].1 != ab[i - 1].1 {
            dp[i][1] += dp[i - 1][1];
            dp[i][1] %= MOD;
        }
    }

    println!("{}", dp[n - 1].iter().sum::<usize>() % MOD);
}
