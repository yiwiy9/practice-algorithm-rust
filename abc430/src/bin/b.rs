use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    let mut set = HashSet::new();

    for start_i in 0..=n - m {
        for start_j in 0..=n - m {
            let mut ans = vec![];
            for diff_i in 0..m {
                let mut row = vec![];
                for diff_j in 0..m {
                    row.push(s[start_i + diff_i][start_j + diff_j]);
                }
                ans.push(row);
            }
            set.insert(ans);
        }
    }

    println!("{}", set.len());
}
