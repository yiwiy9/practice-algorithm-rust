use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1,Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for (u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    if m != n - 1 {
        println!("No");
        return;
    }

    for i in 0..n {
        if graph[i].len() > 2 {
            println!("No");
            return;
        }
    }

    let mut dsu = Dsu::new(n);
    for i in 0..n {
        for &j in &graph[i] {
            dsu.merge(i, j);
        }
    }

    let mut ok = true;
    let leader = dsu.leader(0);
    for i in 0..n {
        if dsu.leader(i) != leader {
            ok = false;
            break;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
