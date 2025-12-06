use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        xyz: [(Usize1,Usize1,i32); m],
    }

    // z < 10^9 < 2^30 ← bitごとに 0 or 1 を決められる

    let mut graph = vec![vec![]; n];
    for &(x, y, z) in &xyz {
        graph[x].push((y, z));
        graph[y].push((x, z));
    }

    let mut ans = vec![0; n];
    let mut values = vec![-1; n];
    for v in 0..n {
        if values[v] != -1 {
            continue;
        }

        let mut connected = vec![];
        values[v] = 0;
        if !dfs(&graph, &mut values, &mut connected, v) {
            println!("-1");
            return;
        }

        for d in 0..30 {
            let cnt = connected
                .iter()
                .filter(|&&u| values[u] & (1 << d) != 0)
                .count();

            if cnt < connected.len() - cnt {
                for &u in &connected {
                    if values[u] & (1 << d) != 0 {
                        ans[u] |= 1 << d;
                    }
                }
            } else {
                for &u in &connected {
                    if values[u] & (1 << d) == 0 {
                        ans[u] |= 1 << d;
                    }
                }
            }
        }
    }

    println!("{}", ans.iter().join(" "));
}

pub fn dfs(
    graph: &Vec<Vec<(usize, i32)>>,
    values: &mut Vec<i32>,
    connected: &mut Vec<usize>,
    v: usize,
) -> bool {
    connected.push(v);
    for &(next_v, z) in &graph[v] {
        if values[next_v] != -1 {
            if values[v] ^ values[next_v] != z {
                return false;
            }
            continue;
        }

        values[next_v] = values[v] ^ z;

        if !dfs(graph, values, connected, next_v) {
            return false;
        }
    }
    true
}
