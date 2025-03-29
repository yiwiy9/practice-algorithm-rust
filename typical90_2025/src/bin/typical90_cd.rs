use proconio::input;
const MOD: u128 = 1_000_000_007;

fn main() {
    input! {
        l: u128,
        r: u128,
    }

    println!("{}", (MOD + calc(r) - calc(l - 1)) % MOD);
}

fn calc(n: u128) -> u128 {
    let mut res = 0;

    let inv_2 = mod_inv(2, MOD);

    let mut digit = 1;
    let mut num = 1;
    loop {
        let next_num = num * 10;
        if next_num > n {
            res += (((n - num + 1) % MOD) * ((num + n) % MOD)) % MOD * inv_2 * digit;
            res %= MOD;
            break;
        } else {
            res += (((next_num - num) % MOD) * ((num + next_num - 1) % MOD)) % MOD * inv_2 * digit;
            res %= MOD;
        }
        num = next_num;
        digit += 1;
    }
    res
}

pub fn mod_pow(base: u128, exp: u128, modulo: u128) -> u128 {
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
pub fn mod_inv(num: u128, modulo: u128) -> u128 {
    mod_pow(num, modulo - 2, modulo)
}
