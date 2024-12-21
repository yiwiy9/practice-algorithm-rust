use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut a = true;
    let mut all_zero = true;
    let mut ans = vec![];
    for &c in s.iter().rev() {
        if c == '0' && a {
            continue;
        } else if all_zero && c == '.' {
            a = false;
            continue;
        }
        a = false;
        all_zero = false;
        ans.push(c);
    }

    println!("{}", ans.iter().rev().join(""));
}
