use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    }

    s.sort_by(|a, b| (a.len()).cmp(&b.len()));

    println!("{}", s.iter().map(|s_i| s_i.iter().join("")).join(""));
}
