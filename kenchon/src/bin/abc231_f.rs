use ac_library::FenwickTree;
use proconio::input;
use std::collections::BTreeMap;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut a_map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (i, &a_i) in a.iter().enumerate() {
        a_map.entry(a_i).or_default().push(i);
    }

    let mut b_unique_sorted = b.clone();
    b_unique_sorted.sort();
    b_unique_sorted.dedup();

    let mut b_cnt_ft = FenwickTree::new(b_unique_sorted.len(), 0);
    let mut ans: usize = 0;
    for idx_vec in a_map.values() {
        let mut idx_vec_map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for &i in idx_vec {
            idx_vec_map.entry(b[i]).or_default().push(i);
        }
        for idx_vec_vec in idx_vec_map.values().rev() {
            let same_ab_cnt = idx_vec_vec.len();
            ans += same_ab_cnt * same_ab_cnt;

            let b_i_idx = b_unique_sorted.lower_bound(&b[idx_vec_vec[0]]);
            ans += same_ab_cnt * b_cnt_ft.sum(b_i_idx..);
            b_cnt_ft.add(b_i_idx, same_ab_cnt);
        }
    }

    println!("{}", ans);
}
