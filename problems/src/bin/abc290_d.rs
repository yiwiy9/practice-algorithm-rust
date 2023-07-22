use num::integer::gcd;
use proconio::input;

/**
 * https://atcoder.jp/contests/abc290/editorial/5767
 * 一周で最大公約数gの倍数がちょうど１回ずつ現れる
 */
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            d: usize,
            k: usize,
        }

        let cnt_per_loop = n / gcd(n, d);
        println!("{}", (k - 1) / cnt_per_loop + ((k - 1) * d) % n);
    }
}
