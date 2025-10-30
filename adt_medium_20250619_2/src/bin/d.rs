use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        k: Chars,
    }

    for i in (0..k.len()).rev() {
        if k[i] != '0' && k[i] != '.' {
            println!("{}", k[..=i].iter().join(""));
            return;
        } else if k[i] == '.' {
            println!("{}", k[..i].iter().join(""));
            return;
        }
    }
    println!("0");
}
