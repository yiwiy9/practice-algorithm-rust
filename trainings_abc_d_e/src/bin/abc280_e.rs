use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        p: usize,
    }

    // モンスターの体力が残りiのときの攻撃回数の期待値
    let mut dp = vec![0; n + 1];
    dp[1] = 1;

    let inv_100 = mod_inv(100, MOD);
    let exp_2 = p * inv_100 % MOD;
    let exp_1 = (100 - p) * inv_100 % MOD;

    for i in 2..=n {
        dp[i] = (dp[i - 2] + 1) * exp_2 % MOD;
        dp[i] += (dp[i - 1] + 1) * exp_1 % MOD;
        dp[i] %= MOD;
    }

    println!("{}", dp[n]);
}

pub fn mod_pow(base: usize, exp: usize, modulo: usize) -> usize {
    if exp == 0 {
        return 1;
    }
    let base = base % modulo;
    let mut result = mod_pow(base * base % modulo, exp / 2, modulo);
    if exp % 2 == 1 {
        result *= base;
        result %= modulo;
    }
    result
}
pub fn mod_inv(num: usize, modulo: usize) -> usize {
    mod_pow(num, modulo - 2, modulo)
}
