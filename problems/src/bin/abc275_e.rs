use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let mod_m = mod_inv(m, MOD);

    let mut dp = vec![0; n + 1];
    dp[n] = 1;
    for _ in 0..k {
        let mut new_dp = vec![0; n + 1];
        new_dp[n] = 1;
        for i in 0..n {
            for j in 1..=m {
                let next_i = if i + j <= n { i + j } else { n - (i + j - n) };
                new_dp[i] += dp[next_i] * mod_m;
                new_dp[i] %= MOD;
            }
        }
        dp = new_dp;
    }

    println!("{}", dp[0]);
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
