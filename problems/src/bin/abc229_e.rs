use ac_library::Dsu;
use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1,Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a].push(b);
    }

    let mut dsu = Dsu::new(n);

    let mut cnt = 0;
    let mut ans = vec![];
    for u in (0..n).rev() {
        ans.push(cnt);

        cnt += 1;

        let mut root_set = BTreeSet::new();
        for &v in &graph[u] {
            root_set.insert(dsu.leader(v));
        }
        cnt -= root_set.len();

        for &v in &graph[u] {
            dsu.merge(u, v);
        }
    }

    println!("{}", ans.iter().rev().join("\n"));
}
