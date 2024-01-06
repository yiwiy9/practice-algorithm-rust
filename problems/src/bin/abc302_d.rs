use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        d: i64,
        a: [i64; n],
        mut b: [i64; m],
    }

    b.sort();

    let mut ans = -1;
    for &a_i in &a {
        let upper_idx = b.upper_bound(&(a_i + d));
        if upper_idx > 0 && b[upper_idx - 1] >= a_i && b[upper_idx - 1] - a_i <= d {
            ans = ans.max(a_i + b[upper_idx - 1]);
        }

        let lower_idx = b.upper_bound(&a_i);
        if lower_idx > 0 && a_i >= b[lower_idx - 1] && a_i - b[lower_idx - 1] <= d {
            ans = ans.max(a_i + b[lower_idx - 1]);
        }
    }

    println!("{}", ans);
}
