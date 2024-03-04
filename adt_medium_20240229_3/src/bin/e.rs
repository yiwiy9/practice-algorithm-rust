use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        xk: [(usize, usize); q],
    }

    let mut cnt_map = BTreeMap::new();
    let mut idx_map = BTreeMap::new();
    for (i, &a_i) in a.iter().enumerate() {
        cnt_map.entry(a_i).and_modify(|cur| *cur += 1).or_insert(1);
        idx_map.insert((a_i, *cnt_map.get(&a_i).unwrap()), (i + 1) as i32);
    }

    for (x, k) in xk {
        println!("{}", idx_map.get(&(x, k)).unwrap_or(&(-1)));
    }
}
