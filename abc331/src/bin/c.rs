use itertools::Itertools;
use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut b = a.clone();
    b.sort();

    let mut map = BTreeMap::new();
    let mut sum = 0;
    for &b_i in b.iter().rev() {
        if !map.contains_key(&b_i) {
            map.insert(b_i, sum);
        }
        sum += b_i;
    }

    println!("{}", a.iter().map(|a_i| map.get(a_i).unwrap()).join(" "));
}
