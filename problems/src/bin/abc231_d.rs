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

    let mut seen = vec![false; n];
    let mut ok = graph.iter().all(|ajd_list| ajd_list.len() < 3);
    for v in 0..n {
        if !seen[v] && !dfs(&graph, &mut seen, v, n) {
            ok = false;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}

pub fn dfs(graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, v: usize, parent: usize) -> bool {
    seen[v] = true;
    for &next_v in &graph[v] {
        if next_v == parent {
            continue;
        }
        if seen[next_v] {
            return false;
        }
        if !dfs(graph, seen, next_v, v) {
            return false;
        }
    }
    true
}
