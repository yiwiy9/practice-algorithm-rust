use ac_library::Dsu;
use proconio::{input, marker::Usize1};
use std::collections::{BTreeMap, BTreeSet};

/**
 * https://atcoder.jp/contests/abc260/tasks/abc260_d
 * https://atcoder.jp/contests/abc260/editorial/4456
 */
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [Usize1; n],
    }

    let mut open_set = BTreeSet::new();
    let mut eaten_map = BTreeMap::new();
    let mut pile_dsu = Dsu::new(n);

    for (i, &p_i) in p.iter().enumerate() {
        if let Some(under) = open_set.range(p_i..).next().copied() {
            open_set.remove(&under);
            pile_dsu.merge(p_i, under);
            if pile_dsu.size(p_i) == k {
                eaten_map.insert(pile_dsu.leader(p_i), i + 1);
            } else {
                open_set.insert(p_i);
            }
        } else if k == 1 {
            eaten_map.insert(p_i, i + 1);
        } else {
            open_set.insert(p_i);
        }
    }

    for i in 0..n {
        println!(
            "{}",
            eaten_map
                .get(&pile_dsu.leader(i))
                .map(|&val| val as i64)
                .unwrap_or(-1)
        );
    }
}
