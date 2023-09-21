use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
        tab: [(usize,usize,usize); q],
    }

    let mut base = 0;
    for (t, a, b) in tab {
        match t {
            1 => s.swap((a - 1 + base) % (2 * n), (b - 1 + base) % (2 * n)),
            2 => base += n,
            _ => unreachable!(),
        }
    }

    println!(
        "{}",
        if base % (2 * n) == 0 {
            s.iter().join("")
        } else {
            s.iter().skip(n).join("") + &s.iter().take(n).join("")
        }
    );
}
