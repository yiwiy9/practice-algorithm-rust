use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        p: usize,
    }

    let mut ans = p - 1;
    ans *= mod_pow(p - 2, n - 1, MOD);
    ans %= MOD;

    println!("{}", ans);
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
