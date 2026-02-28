use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {
        s: Chars,
    }

    let mut map: HashMap<char, usize> = HashMap::new();
    for &c in &s {
        *map.entry(c).or_default() += 1;
    }

    let mut max_cnt = 0;
    for &cnt in map.values() {
        max_cnt = max_cnt.max(cnt);
    }

    println!(
        "{}",
        s.iter()
            .filter(|&&c| map[&c] < max_cnt)
            .map(|&c| c)
            .join("")
    );
}
