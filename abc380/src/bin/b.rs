use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = vec![];
    let mut cnt = 0;
    for (i, &c) in s.iter().skip(1).enumerate() {
        if c == '|' {
            ans.push(cnt);
            cnt = 0;
        } else {
            cnt += 1;
        }
    }

    println!("{}", ans.iter().join(" "));
}
