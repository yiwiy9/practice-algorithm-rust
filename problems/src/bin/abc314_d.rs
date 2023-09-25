use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        mut s: Chars,
        q: usize,
        txc: [(usize,usize,char); q],
    }

    let mut last_case_change = q;
    for (i, &(t, x, c)) in txc.iter().enumerate() {
        match t {
            1 => s[x - 1] = c,
            2 => last_case_change = i,
            3 => last_case_change = i,
            _ => unreachable!(),
        }
    }

    if last_case_change != q {
        let (t, _, _) = txc[last_case_change];
        s = s
            .iter()
            .map(|&c| {
                if t == 2 {
                    c.to_ascii_lowercase()
                } else {
                    c.to_ascii_uppercase()
                }
            })
            .collect();

        for &(_, x, c) in txc.iter().skip(last_case_change + 1) {
            s[x - 1] = c
        }
    }

    println!("{}", s.iter().join(""));
}
