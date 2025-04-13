use proconio::{input, marker::Usize1};
use superslice::*;

/// https://atcoder.jp/contests/abc401/tasks/abc401_f
/// https://atcoder.jp/contests/abc401/editorial/12686
///
/// 1. G1, G2それぞれの木の直径d1, d2を求める
/// 2. G1, G2の各点からの最遠点への距離を求めて、Ai, Bjとすると
/// 3. 追加した辺が連結後の直径となる場合は、Ai + Bj + 1
/// 4. 各追加辺ごとの直径は、max(Ai + Bj + 1, d1, d2)
///
/// 1,2 は 全方位木DPで求める
fn main() {
    input! {
        n1: usize,
        uv1: [(Usize1, Usize1); n1-1],
        n2: usize,
        uv2: [(Usize1, Usize1); n2-1],
    }

    let mut graph1 = vec![vec![]; n1];
    for &(u, v) in &uv1 {
        graph1[u].push(v);
        graph1[v].push(u);
    }
    let mut graph2 = vec![vec![]; n2];
    for &(u, v) in &uv2 {
        graph2[u].push(v);
        graph2[v].push(u);
    }

    let mut a = rerooting(n1, &graph1);
    let mut b = rerooting(n2, &graph2);
    a.sort();
    b.sort();
    let d1 = *a.iter().max().unwrap();
    let d2 = *b.iter().max().unwrap();
    let d = d1.max(d2);

    let mut b_s = vec![0; n2 + 1];
    for i in 0..n2 {
        b_s[i + 1] = b_s[i] + b[i];
    }

    let mut ans = 0;
    for &a_i in &a {
        let upper_idx = if a_i + 1 > d {
            0
        } else {
            b.upper_bound(&(d - a_i - 1))
        };
        ans += upper_idx * d;
        ans += (n2 - upper_idx) * (a_i + 1) + b_s[n2] - b_s[upper_idx];
    }

    println!("{}", ans);
}

/// 各頂点を根としたときの最遠距離（辺のコストは1）を求める
/// 全方位木DP
/// https://ei1333.hateblo.jp/entry/2017/04/10/224413
fn rerooting(n: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut dist = vec![0; n]; // 子方向の最大距離
    dfs1(graph, &mut dist, 0, usize::MAX);

    let mut dp = vec![0; n]; // 親方向から伝播される最大距離
    dfs2(graph, &dist, &mut dp, 0, usize::MAX);

    (0..n).map(|i| dist[i].max(dp[i])).collect()
}

/// 子側の最大距離を集める（ボトムアップ）
fn dfs1(graph: &Vec<Vec<usize>>, dist: &mut Vec<usize>, v: usize, parent: usize) {
    for &child in &graph[v] {
        if child == parent {
            continue;
        }
        dfs1(graph, dist, child, v);
        dist[v] = dist[v].max(dist[child] + 1);
    }
}

/// 親側からの最大距離を子に伝える（トップダウン）
/// プレフィックス・サフィックス最大値で「自分以外の子の最大値」を高速に計算
fn dfs2(graph: &Vec<Vec<usize>>, dist: &Vec<usize>, dp: &mut Vec<usize>, v: usize, parent: usize) {
    // 子とその dist を集める
    let mut child_dists = vec![];
    for &child in &graph[v] {
        if child == parent {
            continue;
        }
        child_dists.push((dist[child] + 1, child)); // +1は辺の重み
    }

    let m = child_dists.len();
    let mut prefix = vec![0; m + 1]; // 左からの最大値
    let mut suffix = vec![0; m + 1]; // 右からの最大値

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
        dp[child] = dp[v].max(use_other) + 1;
        dfs2(graph, dist, dp, child, v);
    }
}
