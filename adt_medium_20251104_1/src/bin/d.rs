use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut ans = vec![];
    for i in 0..n - 1 {
        ans.push(s[i]);
        if s[i] == 'n' && s[i + 1] == 'a' {
            ans.push('y');
        }
    }
    ans.push(s[n - 1]);

    println!("{}", ans.iter().join(""));
}
