use itertools::Itertools as _;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut d: usize,
        s: Chars,
    }

    let mut t = s.clone();
    t.reverse();

    let mut i = 0;
    while d > 0 {
        loop {
            if i >= n {
                break;
            }

            if t[i] == '@' {
                t[i] = '.';
                i += 1;
                break;
            }

            i += 1;
        }

        d -= 1;
    }

    println!("{}", t.iter().rev().join(""));
}
