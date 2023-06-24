use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        x_vec: [usize; q],
    }
    a.sort();
    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }

    for x in &x_vec {
        let upper_i = a.upper_bound(x);

        let mut ans = 0;

        ans += upper_i * x;
        ans -= s[upper_i];

        ans += s[n] - s[upper_i];
        ans -= (n - upper_i) * x;

        println!("{}", ans);
    }
}
