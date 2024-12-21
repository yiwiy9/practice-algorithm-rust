use proconio::input;

/// 有向グラフでクラスカル法は使えない
/// https://atcoder.jp/contests/adt_all_20241218_2/tasks/abc257_d
fn main() {
    input! {
        n: usize,
        xyp: [(i64,i64,i64); n],
    }

    let mut left = -1;
    // let mut right = 1 << 60; オーバーフローするので注意
    let mut right = 4_000_000_001;
    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut graph = vec![vec![]; n];
        for (i, (x_i, y_i, p_i)) in xyp.iter().enumerate() {
            for (j, (x_j, y_j, _)) in xyp.iter().enumerate() {
                if i == j {
                    continue;
                }

                if mid * p_i >= (x_i - x_j).abs() + (y_i - y_j).abs() {
                    graph[i].push(j);
                }
            }
        }

        let mut ok = false;
        for i in 0..n {
            let mut seen = vec![false; n];
            dfs(&graph, &mut seen, i);
            if seen.iter().all(|&s| s) {
                ok = true;
                break;
            }
        }

        if ok {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", right);
}

pub fn dfs(graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, v: usize) {
    seen[v] = true;
    for &next_v in &graph[v] {
        if seen[next_v] {
            continue;
        }
        dfs(graph, seen, next_v);
    }
}
