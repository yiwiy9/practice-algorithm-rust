use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }
    s.push('s');

    println!("{}", s.iter().join(""));
}
