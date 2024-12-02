use itertools::Itertools as _;
use proconio::input;
use std::collections::HashMap;
use superslice::*;

const INF: usize = 1_000_000_000;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut a_idx_map = HashMap::new();
    let mut a_sorted = vec![INF];
    for (i, &a_i) in a.iter().enumerate() {
        if a_sorted.last().unwrap() > &a_i {
            a_sorted.push(a_i);
            a_idx_map.insert(a_i, i);
        }
    }
    a_sorted.sort();

    let mut ans = vec![];
    for &b_i in &b {
        let a_sorted_idx = a_sorted.upper_bound(&b_i);
        if a_sorted_idx == 0 {
            ans.push(-1);
        } else {
            ans.push(a_idx_map[&a_sorted[a_sorted_idx - 1]] as isize + 1);
        }
    }

    println!("{}", ans.iter().join("\n"));
}
