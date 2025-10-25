use itertools::Itertools;
use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        pab: [(usize, usize, usize); n],
        q: usize,
        x: [usize; q],
    }

    let mut dp = vec![(0..=1000).collect_vec(); n + 1];
    for i in (0..n).rev() {
        let (p, a, b) = pab[i];
        for j in 0..=1000 {
            if p >= j {
                dp[i][j] = dp[i + 1][j + a];
            } else {
                dp[i][j] = dp[i + 1][if j > b { j - b } else { 0 }];
            }
        }
    }

    let mut b_s = vec![0; n + 1];
    for i in 0..n {
        b_s[i + 1] = b_s[i] + pab[i].2;
    }

    for &x_i in &x {
        let start_i = if x_i > 1000 {
            b_s.lower_bound(&(x_i - 1000)).min(n)
        } else {
            0
        };

        let start_x = x_i - b_s[start_i].min(x_i);
        if start_i == n {
            println!("{}", start_x);
        } else {
            println!("{}", dp[start_i][start_x]);
        }
    }
}
