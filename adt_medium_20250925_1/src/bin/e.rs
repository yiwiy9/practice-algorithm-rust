use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut same = 0;
    let mut set = HashSet::new();
    for (i, &a_i) in a.iter().enumerate() {
        if i == a_i {
            same += 1;
        } else if a[a_i] == i {
            set.insert((i.min(a_i), i.max(a_i)));
        }
    }

    let same_cnt = if same > 0 { same * (same - 1) / 2 } else { 0 };

    println!("{}", same_cnt + set.len());
}
