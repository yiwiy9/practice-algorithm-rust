use proconio::{input, marker::Usize1};

const INF: usize = 1 << 30;

fn main() {
    input! {
        n: usize,
        k: usize,
        uv: [(Usize1, Usize1); n*k-1],
    }

    if n == 1 && k == 1 {
        println!("Yes");
        return;
    }

    let mut graph = vec![vec![]; n * k];
    let mut degrees = vec![0; n * k];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
        degrees[u] += 1;
        degrees[v] += 1;
    }

    let remain = dfs(
        &graph,
        k,
        n * k,
        degrees.iter().position(|deg| *deg == 1).unwrap(),
    );

    println!("{}", if remain == 0 { "Yes" } else { "No" });
}

pub fn dfs(graph: &Vec<Vec<usize>>, k: usize, parent_v: usize, v: usize) -> usize {
    let mut remains = vec![];
    for &next_v in &graph[v] {
        if next_v == parent_v {
            continue;
        }
        let remain = dfs(graph, k, v, next_v);
        if remain == INF {
            return INF;
        }
        if remain > 0 {
            remains.push(remain);
        }
    }

    remains.sort();

    if remains.is_empty() {
        1 % k
    } else if remains.len() == 1 {
        (1 + remains[0]) % k
    } else if remains.len() == 2 {
        if remains[0] + remains[1] + 1 == k {
            0
        } else {
            INF
        }
    } else {
        INF
    }
}
