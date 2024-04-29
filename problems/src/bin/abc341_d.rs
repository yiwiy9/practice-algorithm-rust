use num::integer::lcm;
use proconio::input;

/**
 * https://atcoder.jp/contests/abc341/editorial/9324
 * 二分探索で解きます
 * xについて広義単調増加なので
 * x/n + x/m - 2 * x/lcm(n,m) >= k
 */
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let lcm_nm = lcm(n, m);

    let mut left = 0;
    let mut right = 1_usize << 60;
    while right - left > 1 {
        let mid = (left + right) / 2;

        let cnt = (mid / n) + (mid / m) - 2 * (mid / lcm_nm);
        if cnt >= k {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", right);
}
