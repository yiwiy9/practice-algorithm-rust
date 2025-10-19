use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let mut map = BTreeMap::new();
    for i in 0..=n - k {
        let t = s[i..i + k].iter().collect::<String>();
        *map.entry(t).or_insert(0) += 1;
    }

    let x = map.iter().map(|(k, v)| v).max().unwrap();

    println!("{}", x);
    println!(
        "{}",
        map.iter()
            .filter(|(k, v)| *v == x)
            .map(|(k, v)| k)
            .join(" ")
    );
}
