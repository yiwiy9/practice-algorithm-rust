use proconio::{input, marker::Usize1};

/**
 * https://twitter.com/e869120/status/1387538790017769474/photo/1
 * 二部グラフの性質を使う
 *   - 木は必ず二部グラフである
 *   - 二部グラフで塗り分けたうちの少なくとも片方はn/2個（半分）以上の頂点を含む
 */
fn main() {
    input! {
        n: usize,
        ab:[(Usize1, Usize1); n-1],
    }

    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut color = vec![0; n];
    dfs(&graph, &mut color, 0, 1);

    let mut red_points = vec![];
    let mut black_points = vec![];
    for (v, &color_v) in color.iter().enumerate() {
        if color_v == 1 {
            red_points.push(v);
        } else {
            black_points.push(v);
        }
    }

    let ans_points = if red_points.len() > black_points.len() {
        red_points
    } else {
        black_points
    };

    let ans = ans_points
        .iter()
        .take(n / 2)
        .map(|x| (x + 1).to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", ans);
}

pub fn dfs(graph: &Vec<Vec<usize>>, color: &mut Vec<usize>, v: usize, cur: usize) {
    color[v] = cur;
    for &next_v in &graph[v] {
        if color[next_v] != 0 {
            continue;
        }
        dfs(graph, color, next_v, 3 - cur);
    }
}
