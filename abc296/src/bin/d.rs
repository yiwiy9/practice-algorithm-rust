use proconio::input;
use std::cmp::min;

/**
 * https://atcoder.jp/contests/abc296/tasks/abc296_d
 * b = ceil(M/a) を求める(a<=b, ceil(M/a)<=N)
 */
fn main() {
    input! {
        n: i64,
        m: i64,
    }

    let inf = 1_i64 << 60;

    let mut ans = inf;
    for a in 1..=n {
        let b = (m + a - 1) / a;
        if b < a {
            break;
        }
        if b <= n {
            ans = min(ans, a * b);
        }
    }

    if ans == inf {
        println!("{}", -1);
    } else {
        println!("{}", ans);
    }
}
