use proconio::{input, marker::Usize1};
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        ix: [(Usize1,usize); q],
    }

    let mut cnt_map = BTreeMap::new();
    for &a_i in &a {
        cnt_map.entry(a_i).and_modify(|cur| *cur += 1).or_insert(1);
    }

    let mut mex_set = BTreeSet::new();
    for i in 0..=n {
        if !cnt_map.contains_key(&i) {
            mex_set.insert(i);
        }
    }

    for &(i, x) in &ix {
        let v = *cnt_map.get(&a[i]).unwrap();
        if v == 1 {
            cnt_map.remove(&a[i]);
            mex_set.insert(a[i]);
        } else {
            cnt_map.insert(a[i], v - 1);
        }

        mex_set.remove(&x);
        cnt_map.entry(x).and_modify(|cur| *cur += 1).or_insert(1);
        a[i] = x;
        println!("{}", mex_set.iter().next().unwrap());
    }
}
