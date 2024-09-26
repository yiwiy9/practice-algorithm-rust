use proconio::{input, marker::Usize1};

/**
 * https://atcoder.jp/contests/abc361/tasks/abc361_e
 * https://atcoder.jp/contests/abc361/editorial/10329
 * => 2*ΣCi - 木の直径 が答え
 *
 * 次数が2以上の頂点を見つけて、そこから最大距離を採用するようにDFSをしたけど、
 * それだと最初に設定した頂点が木の直径に含まれない場合に対応できない
 * => 素直に木の直径を求めるDFSをする
 */
fn main() {
    input! {
        n: usize,
        abc: [(Usize1, Usize1, usize); n-1],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b, c) in &abc {
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    let (_, s) = dfs(&graph, &mut vec![false; n], 0);
    let (d, _) = dfs(&graph, &mut vec![false; n], s);

    println!("{}", abc.iter().map(|&(_, _, c)| c).sum::<usize>() * 2 - d);
}

pub fn dfs(graph: &Vec<Vec<(usize, usize)>>, seen: &mut Vec<bool>, v: usize) -> (usize, usize) {
    seen[v] = true;

    let mut max_dist_v = (0, v);
    for &(next_v, dist) in &graph[v] {
        if seen[next_v] {
            continue;
        }
        let res = dfs(graph, seen, next_v);
        if res.0 + dist > max_dist_v.0 {
            max_dist_v = (res.0 + dist, res.1);
        }
    }

    max_dist_v
}
