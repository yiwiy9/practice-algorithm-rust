use num::integer::gcd;
use proconio::input;

/// https://atcoder.jp/contests/abc290/tasks/abc290_d
/// https://atcoder.jp/contests/abc290/editorial/5767
/// A,B を正整数とし、A と B の最大公約数を g とおく。
/// A=ag,B=bg と表されるとき、a 個の数 0,BmodA,2BmodA,…,(a−1)BmodA の中には、
/// 0,g,2g,…,(a−1)g (すなわち 0 以上 A 未満の g の倍数) がちょうど 1 回ずつ現れる。
fn main() {
    input! {
        t: usize,
        ndk: [(usize,usize,usize); t],
    }

    for &(n, d, k) in &ndk {
        let g = gcd(n, d);
        let a = n / g;
        println!("{}", (k - 1) / a + ((k - 1) * d) % n);
    }
}
