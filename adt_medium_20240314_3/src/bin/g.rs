use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        uv: [(Usize1,Usize1); n-1],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut seen = vec![false; n];
    let mut sub_tree_sizes = vec![vec![]; n];
    dfs(&graph, &mut seen, &mut sub_tree_sizes, 0);

    println!("{}", n - sub_tree_sizes[0].iter().max().unwrap());
}

pub fn dfs(
    graph: &Vec<Vec<usize>>,
    seen: &mut Vec<bool>,
    sub_tree_sizes: &mut Vec<Vec<usize>>,
    v: usize,
) -> usize {
    seen[v] = true;

    let mut sum = 1;
    for &next_v in &graph[v] {
        if seen[next_v] {
            continue;
        }
        let size = dfs(graph, seen, sub_tree_sizes, next_v);
        sum += size;
        sub_tree_sizes[v].push(size);
    }
    sum
}
