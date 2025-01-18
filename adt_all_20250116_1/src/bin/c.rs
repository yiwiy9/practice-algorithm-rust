use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        s: Chars,
    }

    let mut set = HashSet::new();
    for i in 0..s.len() {
        for j in i..s.len() {
            set.insert(s[i..=j].iter().collect::<String>());
        }
    }
    println!("{}", set.len());
}
