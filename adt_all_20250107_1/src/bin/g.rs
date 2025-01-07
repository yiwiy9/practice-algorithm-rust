use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        x: [usize; q],
    }

    a.sort();
    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }

    for &x_i in &x {
        let lower_i = a.lower_bound(&x_i);
        let upper_i = a.upper_bound(&x_i);

        let ans = (x_i * lower_i - s[lower_i]) + (s[n] - s[upper_i] - x_i * (n - upper_i));
        println!("{}", ans);
    }
}
