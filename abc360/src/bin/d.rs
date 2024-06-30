use proconio::{input, marker::Chars};
use superslice::*;

fn main() {
    input! {
        n: usize,
        t: i64,
        s: Chars,
        x: [i64; n],
    }

    let mut right_ants = vec![];
    let mut left_ants = vec![];
    for i in 0..n {
        if s[i] == '1' {
            right_ants.push(x[i]);
        } else {
            left_ants.push(x[i]);
        }
    }

    left_ants.sort();

    let mut ans = 0;
    for right_ant in right_ants {
        ans += left_ants.upper_bound(&(right_ant + 2 * t)) - left_ants.upper_bound(&right_ant);
    }

    println!("{}", ans);
}
