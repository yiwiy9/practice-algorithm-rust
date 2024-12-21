use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map = BTreeMap::new();
    for &x in &a {
        *map.entry(x).or_insert(0) += 1;
    }

    for (_, &cnt) in map.iter().rev() {
        println!("{}", cnt);
    }
    for _ in map.len()..n {
        println!("0");
    }
}
