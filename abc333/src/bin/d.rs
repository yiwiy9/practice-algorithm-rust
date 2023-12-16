use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        uv: [(Usize1,Usize1); n-1],
    }

    let mut graph = vec![vec![]; n];
    for (u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut sub_tree_size = vec![0; n];
    dfs(&graph, &mut sub_tree_size, 0, n);

    let mut max_sub_tree_size = 0;
    for &next_v in &graph[0] {
        max_sub_tree_size = max_sub_tree_size.max(sub_tree_size[next_v]);
    }
    println!("{}", sub_tree_size[0] - max_sub_tree_size);
}

pub fn dfs(graph: &Vec<Vec<usize>>, sub_tree_size: &mut Vec<usize>, v: usize, p: usize) {
    for &next_v in &graph[v] {
        if next_v == p {
            continue;
        }
        dfs(graph, sub_tree_size, next_v, v);
    }

    sub_tree_size[v] = 1;
    for &next_v in &graph[v] {
        if next_v == p {
            continue;
        }
        sub_tree_size[v] += sub_tree_size[next_v]
    }
}
