use proconio::input;
use std::collections::BTreeMap;

const MOD: usize = 998244353;

/**
 * https://atcoder.jp/contests/abc300/tasks/abc300_e
 * https://atcoder.jp/contests/abc300/editorial/6279
 *
 * 確率DP
 * 現時点から答えを満たす確率 = （次の地点から答えを満たす確率）の和
 */
fn main() {
    input! {
        n: usize,
    }

    let mod_inv_5 = mod_inv(5, MOD);

    let mut dp = BTreeMap::new();

    println!("{}", rec(1, n, mod_inv_5, &mut dp));
}

fn rec(s: usize, n: usize, mod_inv_5: usize, dp: &mut BTreeMap<usize, usize>) -> usize {
    if s > n {
        return 0;
    }
    if s == n {
        return 1;
    }
    if dp.contains_key(&s) {
        return dp[&s];
    }

    let mut res = 0;
    for i in 2..=6 {
        res += rec(i * s, n, mod_inv_5, dp);
        res %= MOD;
    }
    dp.insert(s, (res * mod_inv_5) % MOD);
    dp[&s]
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
