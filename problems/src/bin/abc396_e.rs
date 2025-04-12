use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        xyz: [(Usize1, Usize1, i32); m],
    }

    let mut graph = vec![vec![]; n];
    for &(x, y, z) in &xyz {
        graph[x].push((y, z));
        graph[y].push((x, z));
    }

    let mut ans = vec![0; n];
    let mut used_value = vec![-1; n];
    for v in 0..n {
        if used_value[v] != -1 {
            continue;
        }

        used_value[v] = 0; // 仮置き
        let mut connected = vec![];
        if !dfs(&graph, &mut connected, &mut used_value, v) {
            println!("-1");
            return;
        }

        for bit_digit in 0..30 {
            let on_cnt = connected
                .iter()
                .filter(|&&v| used_value[v] & (1 << bit_digit) != 0)
                .count();

            if on_cnt * 2 < connected.len() {
                for &v in &connected {
                    if used_value[v] & (1 << bit_digit) != 0 {
                        ans[v] |= 1 << bit_digit;
                    }
                }
            } else {
                for &v in &connected {
                    if used_value[v] & (1 << bit_digit) == 0 {
                        ans[v] |= 1 << bit_digit;
                    }
                }
            }
        }
    }

    println!("{}", ans.iter().join(" "));
}

pub fn dfs(
    graph: &Vec<Vec<(usize, i32)>>,
    connected: &mut Vec<usize>,
    used_value: &mut Vec<i32>,
    v: usize,
) -> bool {
    connected.push(v);
    for &(next_v, edge_value) in &graph[v] {
        if used_value[next_v] != -1 {
            if used_value[next_v] != (used_value[v] ^ edge_value) {
                return false;
            }
            continue;
        }
        used_value[next_v] = used_value[v] ^ edge_value;
        if !dfs(graph, connected, used_value, next_v) {
            return false;
        }
    }
    true
}
