use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: usize,
        a: [usize; n],
        mut b: [usize; m],
    }

    b.sort();

    let mut b_s = vec![0; m + 1];
    for i in 0..m {
        b_s[i + 1] = b_s[i] + b[i];
    }

    let mut ans = 0;
    for &a_i in &a {
        let max_b = if a_i >= p { 0 } else { p - a_i };
        let max_b_i = b.lower_bound(&max_b);
        let p_cnt = m - max_b_i;

        ans += b_s[max_b_i] + a_i * max_b_i;
        ans += p_cnt * p;
    }

    println!("{}", ans);
}
