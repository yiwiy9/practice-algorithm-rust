use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n-1],
    }

    let mut graph = vec![vec![]; n];
    let mut degree = vec![0; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
        degree[u] += 1;
        degree[v] += 1;
    }

    let start_v = degree.iter().position(|&d| d == 1).unwrap();
    let mut max_cnt = 0;
    dfs(&graph, &mut max_cnt, 0, n, start_v);

    println!("{}", n - max_cnt);
}

pub fn dfs(
    graph: &Vec<Vec<usize>>,
    max_cnt: &mut usize,
    par_child_cnt: usize,
    par_v: usize,
    v: usize,
) -> usize {
    let child_cnt_without_par = graph[v].len() - 1;

    let mut child_cnt_vec = vec![par_child_cnt];
    for &next_v in &graph[v] {
        if next_v == par_v {
            continue;
        }
        child_cnt_vec.push(dfs(graph, max_cnt, child_cnt_without_par, v, next_v));
    }

    child_cnt_vec.sort();
    let mut child_cnt = child_cnt_vec.len();
    for &min_cnt in &child_cnt_vec {
        *max_cnt = std::cmp::max(*max_cnt, 1 + child_cnt * (1 + min_cnt));
        child_cnt -= 1;
    }

    child_cnt_without_par
}
