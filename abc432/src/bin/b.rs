use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut x: Chars,
    }

    x.sort();

    let not_zero_i = x.iter().position(|c| *c != '0').unwrap();

    let mut ans = vec![x[not_zero_i]];
    ans.extend_from_slice(&x[..not_zero_i]);
    if not_zero_i != x.len() - 1 {
        ans.extend_from_slice(&x[not_zero_i + 1..]);
    }

    println!("{}", ans.iter().join(""));
}
