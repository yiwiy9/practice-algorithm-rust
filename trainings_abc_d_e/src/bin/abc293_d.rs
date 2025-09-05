use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        abcd: [(Usize1, char, Usize1, char); m],
    }

    let mut graph = vec![vec![]; 2 * n];

    for i in 0..n {
        graph[i].push(n + i);
        graph[n + i].push(i);
    }
    for (a, b, c, d) in abcd {
        let u = if b == 'B' { a } else { n + a };
        let v = if d == 'B' { c } else { n + c };
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut seen = vec![false; 2 * n];
    let mut cycle_cnt = 0;
    let mut not_cycle_cnt = 0;
    for i in 0..2 * n {
        if seen[i] {
            continue;
        }
        if dfs(&graph, &mut seen, i, 2 * n) {
            cycle_cnt += 1;
        } else {
            not_cycle_cnt += 1;
        }
    }

    println!("{} {}", cycle_cnt, not_cycle_cnt);
}

pub fn dfs(graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, v: usize, p: usize) -> bool {
    seen[v] = true;
    for &next_v in &graph[v] {
        if seen[next_v] {
            if next_v != p {
                return true;
            }
            continue;
        }
        if dfs(graph, seen, next_v, v) {
            return true;
        }
    }
    false
}
