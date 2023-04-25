use proconio::input;

const MOD: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    if n > 2 && k < 3 {
        println!("{}", 0);
        return;
    }

    let mut ans = k;
    if n > 1 {
        ans *= k - 1;
        ans %= MOD;
    }
    if n > 2 {
        ans *= mod_pow(k - 2, n - 2, MOD);
    }
    println!("{}", ans % MOD);
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
