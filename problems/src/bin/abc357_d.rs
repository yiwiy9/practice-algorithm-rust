use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
    }

    let mut n_tmp = n;
    let mut r = 1;
    while n_tmp > 0 {
        r *= 10;
        r %= MOD;
        n_tmp /= 10;
    }

    // Vn = N * (10^NK − 1) / 10^K - 1
    let ans = (n % MOD) * (mod_pow(r, n, MOD) - 1) % MOD * mod_inv(r - 1, MOD) % MOD;

    println!("{}", ans);
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
