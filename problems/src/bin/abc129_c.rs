use proconio::input;
const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let mut b = vec![false; n + 1];
    for &a_i in &a {
        b[a_i] = true;
    }

    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    for i in 0..n {
        if !b[i + 1] {
            dp[i + 1] += dp[i];
            dp[i + 1] %= MOD;
        }
        if i + 2 <= n && !b[i + 2] {
            dp[i + 2] += dp[i];
            dp[i + 2] %= MOD;
        }
    }

    println!("{}", dp[n]);
}
