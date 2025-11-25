use ac_library::modint::ModInt998244353 as Mint;
use proconio::input;

/// https://atcoder.jp/contests/abc414/tasks/abc414_e
/// https://atcoder.jp/contests/abc414/editorial/13450
///
/// Σ(b=1~n) {n - b + 1 - n/b}
/// Σ(b=1~n) {n - b + 1} - Σ(b=1~n) {n/b}
/// => 第１項: N + (N-1) + ... + 1 = n * (n+1) / 2
/// => 第２項: 普通にやったら O(N) だが、商の種類が O(√N) であることに着目して高速化する
fn main() {
    input! {
        n: usize,
    }

    let n_mint = Mint::new(n);

    let mut ans = Mint::new(0);

    let n_b_sum = sum_quotients(n);
    ans += n_mint * (n_mint + 1) / 2;
    ans -= Mint::new(n_b_sum);

    println!("{}", ans.val());
}

/// Σ_{b=1..=n} floor(n / b)
/// ### 概要
/// - S(n) = sum_{b=1..=n} floor(n / b)
/// - 計算量: O(√n)
/// - n は最大 10^18 程度まで安全に扱える
/// - 内部計算は u128 を使用しオーバーフロー安全
pub fn sum_quotients(n: usize) -> u128 {
    let nn = n as u128;
    let mut res: u128 = 0;
    let mut b: u128 = 1;
    while b <= nn {
        let v = nn / b;
        let rb = nn / v;
        let len = rb - b + 1;
        res += v * len;
        b = rb + 1;
    }
    res
}
