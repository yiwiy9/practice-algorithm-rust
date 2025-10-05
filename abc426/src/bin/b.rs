use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut cnt_map = HashMap::new();
    for &c in &s {
        cnt_map.entry(c).and_modify(|cur| *cur += 1).or_insert(1);
    }

    for (k, v) in cnt_map {
        if v == 1 {
            println!("{}", k);
            return;
        }
    }
}
