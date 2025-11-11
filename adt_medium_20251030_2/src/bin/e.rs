use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut cnt = 0;
    let mut t = vec![];
    for &c in &s {
        if c == '"' {
            cnt += 1;
            t.push('"');
        } else if cnt % 2 == 0 && c == ',' {
            t.push('.');
        } else {
            t.push(c);
        }
    }

    println!("{}", t.iter().join(""));
}
