use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    if n == 1 && k == 1 {
        println!("Yes");
        return;
    }

    input! {
        uv: [(Usize1, Usize1); n*k -1],
    }

    let mut graph = vec![vec![]; n * k];
    let mut degrees = vec![0; n * k];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
        degrees[u] += 1;
        degrees[v] += 1;
    }

    let start = degrees
        .iter()
        .enumerate()
        .find(|(_, &deg)| deg == 1)
        .map(|(i, _)| i)
        .unwrap();
    let mut memo = vec![0; n * k];
    if !dfs(&graph, k, &mut memo, n * k, start) {
        println!("No");
        return;
    }

    if memo[start] != 0 {
        println!("No");
        return;
    }

    println!("Yes");
}

pub fn dfs(graph: &Vec<Vec<usize>>, k: usize, memo: &mut Vec<usize>, p: usize, v: usize) -> bool {
    for &next_v in &graph[v] {
        if next_v == p {
            continue;
        }
        if !dfs(graph, k, memo, v, next_v) {
            return false;
        }
    }

    let children = graph[v]
        .iter()
        .filter(|&&next_v| next_v != p && memo[next_v] != 0)
        .map(|&next_v| memo[next_v])
        .collect::<Vec<_>>();

    if children.len() > 2 {
        return false;
    } else if children.len() == 2 {
        if (children[0] + children[1] + 1) % k != 0 {
            return false;
        }
        memo[v] = 0;
    } else if children.len() == 1 {
        memo[v] = (children[0] + 1) % k;
    } else {
        memo[v] = 1 % k;
    }

    true
}
