use std::collections::BTreeSet;

use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut dsu = Dsu::new(n);
    let mut leader_set = vec![BTreeSet::new(); n];

    for u in 0..n {
        for &v in &graph[u] {
            if u < v {
                let u_leader = dsu.leader(u);
                leader_set[u_leader].insert(v);
                continue;
            }

            if dsu.same(u, v) {
                continue;
            }

            let u_leader = dsu.leader(u);
            let v_leader = dsu.leader(v);
            leader_set[v_leader].remove(&u);

            if leader_set[u_leader].len() < leader_set[v_leader].len() {
                for &x in &leader_set[u_leader].clone() {
                    leader_set[v_leader].insert(x);
                }
                dsu.merge(u, v);
                let new_leader = dsu.leader(u);
                leader_set.swap(v_leader, new_leader);
            } else {
                for &x in &leader_set[v_leader].clone() {
                    leader_set[u_leader].insert(x);
                }
                dsu.merge(v, u);
                let new_leader = dsu.leader(u);
                leader_set.swap(u_leader, new_leader);
            }
        }

        let u_leader = dsu.leader(u);

        if dsu.size(u_leader) < u + 1 {
            println!("-1");
        } else {
            println!("{}", leader_set[u_leader].len());
        }
    }
}
