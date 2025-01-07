use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
    }

    let mut dp = vec![1; 10];
    dp[0] = 0;

    for _ in 1..n {
        let mut next_dp = vec![0; 10];

        for i in 1..=9 {
            next_dp[i] += dp[i];
            next_dp[i] %= MOD;

            if i > 1 {
                next_dp[i - 1] += dp[i];
                next_dp[i - 1] %= MOD;
            }
            if i < 9 {
                next_dp[i + 1] += dp[i];
                next_dp[i + 1] %= MOD;
            }
        }

        dp = next_dp;
    }

    println!("{}", dp.iter().skip(1).fold(0, |acc, &x| (acc + x) % MOD));
}
