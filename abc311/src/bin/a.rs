use proconio::{input, marker::Chars};
use std::collections::BTreeSet;

fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    let mut set = BTreeSet::new();
    for (i, c) in s.iter().enumerate() {
        set.insert(c);
        if set.len() == 3 {
            println!("{}", i + 1);
            return;
        }
    }
}
