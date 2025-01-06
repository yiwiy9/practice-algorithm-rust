use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mod_n = mod_inv(n, MOD);

    let mut dp = vec![0; n + 1];
    let mut sum = 0;
    for i in (0..n).rev() {
        let cur = mod_n * (dp[i + 1] + a[i]) % MOD;
        sum += cur;
        sum %= MOD;
        dp[i] = sum;
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
