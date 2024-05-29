use num::integer::gcd;
use proconio::input;

/**
 * https://atcoder.jp/contests/adt_all_20240529_3/tasks/abc276_d
 * https://atcoder.jp/contests/adt_all_20240529_3/editorial/5167
 */
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut g = 0;
    for &a_i in &a {
        g = gcd(g, a_i);
    }

    let mut ans = 0;
    for &a_i in &a {
        let mut cur = a_i / g;

        while cur % 2 == 0 {
            cur /= 2;
            ans += 1;
        }

        while cur % 3 == 0 {
            cur /= 3;
            ans += 1;
        }

        if cur != 1 {
            println!("{}", -1);
            return;
        }
    }

    println!("{}", ans);
}
