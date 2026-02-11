use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
        a: [usize; n],
    }

    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }

    let mut ans: usize = 0;
    for l in 0..n {
        let mut s_r_min = s.lower_bound(&(s[l] + m));
        s_r_min = s_r_min.max(l + k);
        if s_r_min <= n {
            ans += n - s_r_min + 1;
        }
    }

    println!("{}", ans);
}
