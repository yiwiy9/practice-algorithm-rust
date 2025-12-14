use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let num = n - s.len();
    let mut ans = std::iter::repeat('o').take(num).collect_vec();
    ans.extend(s.iter());

    println!("{}", ans.iter().join(""));
}
