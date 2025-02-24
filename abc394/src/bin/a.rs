use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    println!("{}", s.iter().filter(|&&c| c == '2').map(|c| c).join(""));
}
