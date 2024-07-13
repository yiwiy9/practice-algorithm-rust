use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    let mut dsu = Dsu::new(n);
    for (u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
        dsu.merge(u, v);
    }

    let mut color = vec![-1; n];
    for i in 0..n {
        if color[i] != -1 {
            continue;
        }
        if !dfs(&graph, &mut color, i, 0) {
            println!("0");
            return;
        }
    }

    let mut black_cnt = vec![0; n];
    let mut white_cnt = vec![0; n];
    for i in 0..n {
        if color[i] == 0 {
            black_cnt[dsu.leader(i)] += 1;
        } else {
            white_cnt[dsu.leader(i)] += 1;
        }
    }

    let mut ans = 0;
    for i in 0..n {
        ans += n - dsu.size(i);
        if color[i] == 0 {
            ans += white_cnt[dsu.leader(i)] - graph[i].len();
        } else {
            ans += black_cnt[dsu.leader(i)] - graph[i].len();
        }
    }

    println!("{}", ans / 2);
}

fn dfs(graph: &Vec<Vec<usize>>, color: &mut Vec<i32>, v: usize, cur: i32) -> bool {
    color[v] = cur;
    for &next_v in &graph[v] {
        if color[next_v] != -1 {
            if color[next_v] == cur {
                return false;
            }
            continue;
        }
        if !dfs(graph, color, next_v, cur ^ 1) {
            return false;
        }
    }
    true
}
