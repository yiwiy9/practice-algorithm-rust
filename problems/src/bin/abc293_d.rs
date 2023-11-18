use ac_library::Dsu;
use proconio::{input, marker::Usize1};
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        abcd: [(Usize1,char,Usize1,char); m],
    }

    let mut dsu = Dsu::new(n);

    let mut cycle_cnt = 0;
    for &(a, _, c, _) in &abcd {
        if dsu.leader(a) == dsu.leader(c) {
            cycle_cnt += 1;
            continue;
        }
        dsu.merge(a, c);
    }

    let mut set = BTreeSet::new();
    for i in 0..n {
        set.insert(dsu.leader(i));
    }

    println!("{} {}", cycle_cnt, set.len() - cycle_cnt);
}
