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
        let lower_i = if a_i < p {
            b.upper_bound(&(p - a_i))
        } else {
            0
        };

        ans += a_i * (lower_i) + b_s[lower_i];
        ans += p * (m - lower_i);
    }

    println!("{}", ans);
}
