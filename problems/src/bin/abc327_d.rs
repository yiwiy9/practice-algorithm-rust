use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
        b: [Usize1; m],
    }

    let mut graph = vec![vec![]; n];
    for i in 0..m {
        if a[i] != b[i] {
            graph[a[i]].push(b[i]);
            graph[b[i]].push(a[i]);
        } else {
            println!("No");
            return;
        }
    }

    let mut ok = true;
    let mut color = vec![-1; n];
    for i in 0..n {
        if color[i] != -1 {
            continue;
        }
        if !dfs(&graph, &mut color, i, 0) {
            ok = false;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}

// 二部グラフ判定
pub fn dfs(graph: &Vec<Vec<usize>>, color: &mut Vec<i32>, v: usize, cur: i32) -> bool {
    color[v] = cur;
    for &next_v in &graph[v] {
        if color[next_v] != -1 {
            if color[next_v] == cur {
                return false;
            }
            continue;
        }
        if !dfs(graph, color, next_v, 1 - cur) {
            return false;
        }
    }
    true
}
