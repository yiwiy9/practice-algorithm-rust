use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let mut dp = vec![0; k + 1];
    dp[0] = 1;
    for _ in 0..n {
        let mut new_dp = vec![0; k + 1];
        for i in 0..=k {
            for j in 1..=m {
                if i + j <= k {
                    new_dp[i + j] += dp[i];
                    new_dp[i + j] %= MOD;
                }
            }
        }
        dp = new_dp;
    }

    println!("{}", dp.iter().fold(0, |acc, &x| (acc + x) % MOD));
}
