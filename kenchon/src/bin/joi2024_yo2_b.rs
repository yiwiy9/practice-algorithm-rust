use proconio::{input, marker::Usize1};
use std::collections::BTreeMap;
use superslice::*;

fn main() {
    input! {
        n: usize,
        _m: usize,
        q: usize,
        pa: [(usize,usize); n],
        tlr: [(usize,Usize1,Usize1); q],
    }

    let mut p_s = vec![0; n + 1];
    for i in 0..n {
        p_s[i + 1] = p_s[i] + pa[i].0;
    }

    let mut map = BTreeMap::new();
    for (i, &(p, a)) in pa.iter().enumerate() {
        let (idx_vec, sales_vec) = map.entry(a).or_insert((vec![], vec![0]));
        idx_vec.push(i);
        sales_vec.push(p / 2 + sales_vec.last().unwrap());
    }

    for &(t, l, r) in &tlr {
        let mut ans = p_s[r + 1] - p_s[l];

        if let Some((idx_vec, sales_vec)) = map.get(&t) {
            let left_i = idx_vec.lower_bound(&l);
            let right_i = idx_vec.lower_bound(&(r + 1));
            ans -= sales_vec[right_i] - sales_vec[left_i];
        }

        println!("{}", ans);
    }
}
