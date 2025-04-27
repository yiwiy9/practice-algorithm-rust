use itertools::Itertools as _;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut t = vec![];
    let mut cnt = 0;
    for &c in &s {
        if c == '"' {
            cnt += 1;
        } else if cnt % 2 == 0 && c == ',' {
            t.push('.');
            continue;
        }
        t.push(c);
    }

    println!("{}", t.iter().join(""));
}
