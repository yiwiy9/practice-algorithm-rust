use itertools::Itertools;
use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        l: usize,
        a: [usize; n],
    }

    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }

    let mut ans = vec![(0, 0); n];
    for i in (0..n).rev() {
        let upper_i = s.upper_bound(&(s[i] + l));
        if upper_i == n + 1 {
            ans[i] = (1, s[n] - s[i]);
        } else {
            ans[i] = (ans[upper_i - 1].0 + 1, ans[upper_i - 1].1);
        }
    }

    println!(
        "{}",
        ans.iter()
            .map(|&(cnt, last)| format!("{} {}", cnt, last))
            .join("\n")
    );
}
