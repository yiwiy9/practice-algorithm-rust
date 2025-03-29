use proconio::{input_interactive, marker::Usize1};
use std::collections::HashSet;

/// https://docs.rs/proconio/latest/proconio/macro.input_interactive.html
/// インタラクティブな入力を受け取るためのマクロ: input_interactive
///
/// 出力する際も flush は必要なかったみたい
/// （input_interactiveは関係ないが、printlnで事足りるようだ）
fn main() {
    input_interactive! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut even_cycles = vec![];
    for u in 0..n {
        for v in u + 1..n {
            if graph[u].contains(&v) {
                continue;
            }
            let mut seen = vec![false; n];
            if dfs(&graph, v, &mut seen, u, 0) {
                even_cycles.push((u + 1, v + 1));
            }
        }
    }

    println!(
        "{}",
        if even_cycles.len() % 2 == 0 {
            "Second"
        } else {
            "First"
        }
    );

    let mut used_set = HashSet::new();
    if even_cycles.len() % 2 == 0 {
        input_interactive! {
            mut i: i64,
            mut j: i64,
        }
        if i == -1 && j == -1 {
            return;
        }
        if i > j {
            std::mem::swap(&mut i, &mut j);
        }
        used_set.insert((i as usize, j as usize));
    }

    let mut i = 0;
    loop {
        while i < even_cycles.len() {
            let (u, v) = even_cycles[i];
            if !used_set.contains(&(u, v)) {
                println!("{} {}", u, v);
                used_set.insert((u, v));
                break;
            }
            i += 1;
        }

        input_interactive! {
            mut i: i64,
            mut j: i64,
        }
        if i == -1 && j == -1 {
            return;
        }
        if i > j {
            std::mem::swap(&mut i, &mut j);
        }
        used_set.insert((i as usize, j as usize));
    }
}

pub fn dfs(
    graph: &Vec<Vec<usize>>,
    goal: usize,
    seen: &mut Vec<bool>,
    v: usize,
    cnt: usize,
) -> bool {
    seen[v] = true;

    if v == goal {
        return cnt % 2 == 1;
    }

    for &next_v in &graph[v] {
        if seen[next_v] {
            continue;
        }
        if dfs(graph, goal, seen, next_v, cnt + 1) {
            return true;
        }
    }

    false
}
