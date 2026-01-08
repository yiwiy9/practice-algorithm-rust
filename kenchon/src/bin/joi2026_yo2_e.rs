use std::collections::BTreeSet;

use ac_library::Dsu;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/joi2026yo2/tasks/joi2026_yo2_e
/// https://www2.ioi-jp.org/joi/2025/2026-yo2/2026-yo2-t5-review.pdf
/// => 頂点倍加する必要がある
/// => 始点から数えて奇数番目の頂点 or 偶数番目の頂点で２頂点作成する
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1,Usize1); m],
    }

    let mut leader_set = (0..2 * n).map(|i| BTreeSet::from([i % n])).collect_vec();
    let mut dsu = Dsu::new(2 * n);
    for &(a, b) in &ab {
        if dsu.same(a, b + n) {
            continue;
        }

        let mut a_idx = dsu.leader(a);
        let mut b_idx = dsu.leader(b + n);
        if leader_set[a_idx].len() > leader_set[b_idx].len() {
            std::mem::swap(&mut a_idx, &mut b_idx);
        }

        let a_set = leader_set[a_idx].clone();
        for &a_v in &a_set {
            leader_set[b_idx].insert(a_v);
        }

        dsu.merge(a, b + n); // a<b a:odd,b:even(+n)
        leader_set.swap(b_idx, dsu.leader(a));
    }

    println!(
        "{}",
        (0..n)
            .map(|i| n - leader_set[dsu.leader(i)].len())
            .join("\n")
    );
}
