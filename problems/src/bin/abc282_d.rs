use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/abc282/tasks/abc282_d
/// https://atcoder.jp/contests/abc282/editorial/5397
/// 余事象で考える癖をつけると楽に解ける
fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for (u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut color = vec![-1; n];
    let mut ans = n * (n - 1) / 2 - m;
    for v in 0..n {
        if color[v] != -1 {
            continue;
        }

        let (b_cnt, w_cnt) = dfs(&graph, &mut color, v, 0);
        if b_cnt == -1 {
            println!("0");
            return;
        }
        let b_cnt_u = b_cnt as usize;
        let w_cnt_u = w_cnt as usize;
        ans -= b_cnt_u * (b_cnt_u - 1) / 2;
        ans -= w_cnt_u * (w_cnt_u - 1) / 2;
    }

    println!("{}", ans);
}

fn dfs(graph: &Vec<Vec<usize>>, color: &mut Vec<i32>, v: usize, cur: i32) -> (i64, i64) {
    let mut res = if cur == 0 { (1, 0) } else { (0, 1) };
    color[v] = cur;

    for &next_v in &graph[v] {
        if color[next_v] != -1 {
            if color[next_v] == cur {
                return (-1, -1);
            }
            continue;
        }

        let (b_cnt, w_cnt) = dfs(graph, color, next_v, cur ^ 1);
        if b_cnt == -1 {
            return (-1, -1);
        }

        res.0 += b_cnt;
        res.1 += w_cnt;
    }

    res
}
