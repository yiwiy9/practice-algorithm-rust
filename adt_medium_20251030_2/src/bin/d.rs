use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        l: usize,
        r: usize,
        s: Chars,
    }

    let mut t = s[..l - 1].to_vec();
    t.extend(s[l - 1..r].iter().rev());
    t.extend(s[r..].iter());

    println!("{}", t.iter().join(""));
}
