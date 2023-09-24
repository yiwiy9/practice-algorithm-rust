use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }

    b.sort();

    let mut s_b = vec![0; m + 1];
    for i in 0..m {
        s_b[i + 1] = s_b[i] + b[i];
    }

    let mut ans = 0;
    for &a_i in &a {
        if a_i >= p {
            ans += m * p;
        } else {
            let b_index = b.lower_bound(&(p - a_i));
            ans += a_i * b_index + s_b[b_index];
            ans += (m - b_index) * p;
        }
    }

    println!("{}", ans);
}
