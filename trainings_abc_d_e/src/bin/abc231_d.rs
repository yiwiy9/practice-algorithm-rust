use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1,Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    if graph.iter().any(|v| v.len() > 2) {
        println!("No");
        return;
    }

    let mut dsu = Dsu::new(n);
    let mut ok = true;
    for &(a, b) in &ab {
        if dsu.same(a, b) {
            ok = false;
            break;
        }
        dsu.merge(a, b);
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
