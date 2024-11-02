use ac_library::Dsu;
use proconio::{input, marker::Usize1};
use std::mem::swap;

fn main() {
    input! {
        n: usize,
        uv: [(Usize1,Usize1); n-1],
    }

    let mut degrees = vec![0; n];
    for &(u, v) in &uv {
        degrees[u] += 1;
        degrees[v] += 1;
    }

    let mut deg2_cnt = vec![0; n];
    let mut dsu = Dsu::new(n);
    for &(u, v) in &uv {
        if degrees[u] == 3 && degrees[v] == 3 {
            let mut u_leader = dsu.leader(u);
            let mut v_leader = dsu.leader(v);
            dsu.merge(u, v);
            if dsu.leader(u) != u_leader {
                swap(&mut u_leader, &mut v_leader);
            }
            deg2_cnt[u_leader] += deg2_cnt[v_leader];
            deg2_cnt[v_leader] = 0;
        } else if degrees[u] == 2 && degrees[v] == 3 {
            deg2_cnt[dsu.leader(v)] += 1;
        } else if degrees[u] == 3 && degrees[v] == 2 {
            deg2_cnt[dsu.leader(u)] += 1;
        }
    }

    println!(
        "{}",
        deg2_cnt
            .iter()
            .map(|&x| if x > 0 { x * (x - 1) / 2 } else { 0 })
            .sum::<usize>()
    );
}
