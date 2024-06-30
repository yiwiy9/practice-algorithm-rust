use proconio::input;
const MOD: usize = 998244353;

/**
 * https://atcoder.jp/contests/abc360/tasks/abc360_e
 * https://atcoder.jp/contests/abc360/editorial/10310
 * 操作の対称性を考えると、ある時点で黒いボールが最も左にある確率を p としたとき、
 * 左から 2,3,…,N 番目にある確率はどれも N−1/1−p です。そのため、K回操作をした後に黒いボールが最も左にある確率が求まれば、この問題を解けます。
 * => N が馬鹿デカくても上手くいくのはこの対称性があるから
 *
 * dp[i] := i回操作をしたとき黒いボールが最も左にある確率
 *
 * - p := 黒いボールが最も左にある状態から1回操作して、黒いボールが最も左以外のどこかに移動する確率
 * - q := 黒いボールが最も左以外のどこかにある状態から1回操作して、黒いボールが最も左に移動する確率
 * => p = 2(n-1) / n^2, q = 2 / n^2
 *
 * dp[i] = (1 - p) * dp[i-1] + q * (1 - dp[i-1])
 */
fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let n_sq_inv = mod_inv(n * n, MOD);
    let p = 2 * (n - 1) % MOD * n_sq_inv % MOD;
    let q = 2 * n_sq_inv % MOD;

    let mut dp = vec![0; k + 1];
    dp[0] = 1;

    for i in 1..=k {
        dp[i] = ((1 + MOD - p) % MOD * dp[i - 1]) % MOD;
        dp[i] += q * ((1 + MOD - dp[i - 1]) % MOD) % MOD;
        dp[i] %= MOD;
    }

    // 1-p/n-1 * (2+...+n) => 1-p/n-1 * (n(n+1)/2 - 1) => 1-p/n-1 * (n-1)(n+2) / 2
    let mut not_one_exp = (1 + MOD - dp[k]) % MOD;
    not_one_exp *= (n + 2) % MOD * mod_inv(2, MOD) % MOD;
    not_one_exp %= MOD;

    let ans = (dp[k] + not_one_exp) % MOD;

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
