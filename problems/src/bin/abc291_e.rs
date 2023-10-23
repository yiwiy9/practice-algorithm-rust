use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(x, y) in &xy {
        graph[x].push(y);
    }

    let mut dist = vec![-1; n];
    let mut order = vec![];
    let mut longest_path = 0;
    for v in 0..n {
        longest_path = longest_path.max(dfs(&graph, &mut dist, &mut order, v));
    }

    if longest_path as usize != n - 1 {
        println!("No");
        return;
    }

    let mut num = 1;
    let mut ans = vec![0; n];
    for &v in order.iter().rev() {
        ans[v] = num;
        num += 1;
    }

    println!("Yes");
    println!("{}", ans.iter().join(" "));
}

pub fn dfs(graph: &Vec<Vec<usize>>, dist: &mut Vec<i32>, order: &mut Vec<usize>, v: usize) -> i32 {
    if dist[v] != -1 {
        return dist[v];
    }

    let mut max_dist = 0;
    for &next_v in &graph[v] {
        max_dist = max_dist.max(dfs(graph, dist, order, next_v) + 1);
    }

    // トポロジカルソート順
    order.push(v);

    dist[v] = max_dist;
    dist[v]
}
