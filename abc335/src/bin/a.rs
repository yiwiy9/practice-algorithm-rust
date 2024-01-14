use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }
    let a = s.len();
    s[a - 1] = '4';

    println!("{}", s.iter().join(""));
}
