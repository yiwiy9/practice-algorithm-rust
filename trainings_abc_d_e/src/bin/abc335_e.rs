use ac_library::Dsu;
use proconio::{input, marker::Usize1};
use std::mem::swap;

/// https://atcoder.jp/contests/abc335/tasks/abc335_e
/// https://atcoder.jp/contests/abc335/editorial/9037
/// 値はDPで計算できそうだけど、サイクルがあるとDPできない
/// => DAG（依存が一方向）を作れたら、DPできる
///
/// 1. 頂点の値が同じ辺は一つの点としてまとめる (Union Find)
/// 2. 頂点の値が小→大で有向辺を引く
///    => これで、DAGになる
/// 3. 連結成分ごとに a の値でソートして DP
///    => この DP が案外思い付かない
///    => DAG になったので、a の値が小さいところから上位の score を更新できる
///    => すべての緩和処理が終わった後の頂点 n の score が答え（頂点 n に向けて緩和していくのではない）
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        uv: [(Usize1, Usize1); m],
    }

    let mut dag_edges = vec![vec![]; 200_010];
    let mut dsu = Dsu::new(n);
    for &(mut u, mut v) in &uv {
        if a[u] == a[v] {
            dsu.merge(u, v);
        } else {
            if a[u] > a[v] {
                swap(&mut u, &mut v);
            }
            dag_edges[a[u]].push((u, v));
        }
    }

    let mut dp = vec![0; n];
    dp[dsu.leader(0)] = 1;
    for dag_edge in &dag_edges {
        for &(u, v) in dag_edge {
            let u_leader = dsu.leader(u);
            let v_leader = dsu.leader(v);
            if dp[u_leader] > 0 {
                dp[v_leader] = dp[v_leader].max(dp[u_leader] + 1);
            }
        }
    }

    println!("{}", dp[dsu.leader(n - 1)]);
}

/// 「サイクルありのグラフ」なので DFS で 帰りがけ順に DP できない
/// => その名のとおり、今回の問題設定では 木DP できない
///
/// サイクルがあると、seen[next_v] && scores[next_v] == -1 の continue で scoresが一生更新されない点が出てくる
pub fn dfs(
    n: usize,
    graph: &Vec<Vec<usize>>,
    a: &Vec<usize>,
    seen: &mut Vec<bool>,
    scores: &mut Vec<i64>,
    v: usize,
) {
    seen[v] = true;

    if v == n - 1 {
        scores[v] = 1;
        return;
    };

    let mut score = 0;
    for &next_v in &graph[v] {
        if seen[next_v] {
            if scores[next_v] == -1 {
                continue;
            }
        } else {
            dfs(n, graph, a, seen, scores, next_v);
        }

        if scores[next_v] == 0 {
            continue;
        }

        if a[v] < a[next_v] {
            score = score.max(scores[next_v] + 1);
        } else if a[v] == a[next_v] {
            score = score.max(scores[next_v]);
        }
    }

    scores[v] = score;
}
