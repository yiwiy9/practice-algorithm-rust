use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut ans = vec![];
    let mut j = 0;
    for &s_i in &s {
        while s_i != t[j] {
            j += 1;
        }
        ans.push(j + 1);
        j += 1;
    }
    println!("{}", ans.iter().join(" "));
}
