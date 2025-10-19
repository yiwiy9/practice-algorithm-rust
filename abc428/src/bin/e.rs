use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n-1],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let dist = rerooting(n, &graph);

    println!("{}", dist.iter().map(|(_, v)| v + 1).join("\n"));
}

/// 各頂点を根としたときの最遠距離（辺のコストは1）を求める
/// 全方位木DP
/// https://ei1333.hateblo.jp/entry/2017/04/10/224413
fn rerooting(n: usize, graph: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut dist = (0..n).map(|i| (0, i)).collect_vec(); // 子方向の最大距離
    dfs1(graph, &mut dist, 0, usize::MAX);

    let mut dp = (0..n).map(|i| (0, i)).collect_vec(); // 親方向から伝播される最大距離
    dfs2(graph, &dist, &mut dp, 0, usize::MAX);

    (0..n).map(|i| dist[i].max(dp[i])).collect()
}

/// 子側の最大距離を集める（ボトムアップ）
fn dfs1(graph: &Vec<Vec<usize>>, dist: &mut Vec<(usize, usize)>, v: usize, parent: usize) {
    for &child in &graph[v] {
        if child == parent {
            continue;
        }
        dfs1(graph, dist, child, v);
        dist[v] = dist[v].max((dist[child].0 + 1, dist[child].1));
    }
}

/// 親側からの最大距離を子に伝える（トップダウン）
/// プレフィックス・サフィックス最大値で「自分以外の子の最大値」を高速に計算
fn dfs2(
    graph: &Vec<Vec<usize>>,
    dist: &Vec<(usize, usize)>,
    dp: &mut Vec<(usize, usize)>,
    v: usize,
    parent: usize,
) {
    // 子とその dist を集める
    let mut child_dists = vec![];
    for &child in &graph[v] {
        if child == parent {
            continue;
        }
        child_dists.push(((dist[child].0 + 1, dist[child].1), child)); // +1は辺の重み
    }

    let m = child_dists.len();
    let mut prefix = vec![(0, 0); m + 1]; // 左からの最大値
    let mut suffix = vec![(0, 0); m + 1]; // 右からの最大値

    // 各子に対し「他の子からの最大値」を高速に求めるための準備
    for i in 0..m {
        prefix[i + 1] = prefix[i].max(child_dists[i].0);
    }
    for i in (0..m).rev() {
        suffix[i] = suffix[i + 1].max(child_dists[i].0);
    }

    for (i, &(_, child)) in child_dists.iter().enumerate() {
        // 子に渡すべき「他の子方向の最大距離」
        let use_other = prefix[i].max(suffix[i + 1]);
        // 親方向と他の子方向の最大値に +1 して渡す
        let max_dp = dp[v].max(use_other);
        dp[child] = (max_dp.0 + 1, max_dp.1);
        dfs2(graph, dist, dp, child, v);
    }
}
