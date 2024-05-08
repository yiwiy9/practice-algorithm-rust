use proconio::{input, marker::Usize1};
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1,Usize1); m],
        q: usize,
        xk: [(Usize1,i64); q],
    }

    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    for (x, k) in xk {
        let mut reached = BTreeSet::new();
        dfs(&graph, &mut reached, x, k);
        println!("{}", reached.iter().map(|&x| x + 1).sum::<usize>());
    }
}

fn dfs(graph: &Vec<Vec<usize>>, reached: &mut BTreeSet<usize>, cur: usize, k: i64) {
    if k == -1 {
        return;
    }
    reached.insert(cur);

    for &next in &graph[cur] {
        dfs(graph, reached, next, k - 1);
    }
}
