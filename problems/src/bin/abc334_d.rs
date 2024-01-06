use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut r: [usize; n],
        x: [usize; q],
    }
    r.sort();
    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + r[i];
    }

    for x_j in &x {
        let s_idx = s.upper_bound(x_j);
        println!("{}", s_idx - 1);
    }
}
