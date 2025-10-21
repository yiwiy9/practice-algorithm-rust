use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        d: usize,
        s: Chars,
    }

    let mut cnt = 0;
    let mut t = vec![];
    for &c in s.iter().rev() {
        if c == '@' && cnt < d {
            t.push('.');
            cnt += 1;
        } else {
            t.push(c);
        }
    }

    println!("{}", t.iter().rev().join(""));
}
