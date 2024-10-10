use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut map = BTreeMap::new();
    for s_i in &s {
        if let Some(x) = map.get_mut(s_i) {
            println!("{}({})", s_i.iter().join(""), x);
            *x += 1;
        } else {
            println!("{}", s_i.iter().join(""));
            map.insert(s_i.clone(), 1);
        }
    }
}
