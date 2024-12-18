use ac_library::Dsu;
use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [Usize1; n],
    }

    let mut leader_ans = vec![-1; n];
    let mut dsu = Dsu::new(n);

    let mut field = BTreeSet::new();
    for (i, &p_i) in p.iter().enumerate() {
        if let Some(before_num) = field.range(p_i..).next().copied() {
            field.remove(&before_num);
            dsu.merge(before_num, p_i);
        }

        if dsu.size(p_i) == k {
            leader_ans[dsu.leader(p_i)] = (i + 1) as i32;
        } else {
            field.insert(p_i);
        }
    }

    println!("{}", (0..n).map(|i| leader_ans[dsu.leader(i)]).join("\n"));
}
