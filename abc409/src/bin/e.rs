use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        x: [i64; n],
        uvw: [(Usize1, Usize1, i64); n-1],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, w) in &uvw {
        graph[u].push((v, w));
        graph[v].push((u, w));
    }

    let (cost, _) = dfs(&graph, &x, n, 0);

    println!("{}", cost);
}

pub fn dfs(graph: &Vec<Vec<(usize, i64)>>, x: &Vec<i64>, par: usize, v: usize) -> (i64, i64) {
    let mut cost_sum = 0;
    let mut energy_sum = x[v];
    for &(to, w) in &graph[v] {
        if to == par {
            continue;
        }
        let (sub_cost, sub_energy) = dfs(graph, x, v, to);
        cost_sum += sub_cost + sub_energy.abs() * w;
        energy_sum += sub_energy;
    }
    (cost_sum, energy_sum)
}
