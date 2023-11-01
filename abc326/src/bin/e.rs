use proconio::input;
const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mod_inv_n = mod_inv(n, MOD);

    let mut ex_sum = 0;
    let mut dp = vec![0; n + 1];
    for i in (0..n).rev() {
        dp[i] = a[i] + ex_sum;
        dp[i] %= MOD;
        dp[i] *= mod_inv_n;
        dp[i] %= MOD;

        ex_sum += dp[i];
        ex_sum %= MOD;
    }

    println!("{}", ex_sum);
}

pub fn mod_pow(base: usize, exp: usize, modulo: usize) -> usize {
    if exp == 0 {
        return 1;
    }
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
