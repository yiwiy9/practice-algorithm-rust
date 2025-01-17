use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut rev = vec![];
    for &c in s.iter().rev() {
        if c == '.' {
            break;
        }
        rev.push(c);
    }

    println!("{}", rev.iter().rev().join(""));
}
