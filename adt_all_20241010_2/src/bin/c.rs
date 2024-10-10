use proconio::{input, marker::Usize1};
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
        q: usize,
        ab: [(Usize1,Usize1); q],
    }

    let mut map = BTreeMap::new();
    for i in 0..n {
        map.insert(p[i], i);
    }

    for (a, b) in ab {
        let a_i = map.get(&a).unwrap();
        let b_i = map.get(&b).unwrap();
        println!("{}", if a_i < b_i { a + 1 } else { b + 1 });
    }
}
