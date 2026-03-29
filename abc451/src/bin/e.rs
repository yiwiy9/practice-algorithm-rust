use proconio::input;

const INF: usize = 1 << 60;

/// https://atcoder.jp/contests/abc451/tasks/abc451_e
/// https://atcoder.jp/contests/abc451/editorial/18053
///
/// 方針:
/// - 距離行列 A から、頂点 1 を根とした各頂点 i (i != 1) の親を復元する。
/// - 頂点 j が i の祖先である必要十分条件は
///   A[1][j] + A[j][i] == A[1][i]
///   である。
/// - よって各 i について、この条件を満たす頂点のうち
///   A[i][j] が最小のものを親とする。
/// - こうして得た親子関係から候補木を構築し、木上の全点対距離を DFS で計算して
///   入力の距離行列と完全一致するかを確認する。
/// - 一致すれば Yes、そうでなければ No。
///
/// 計算量:
/// - 親の復元: O(N^2)
/// - 検証 DFS: 各頂点を始点に O(N)
/// - 全体: O(N^2)
fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![vec![0; n]; n];
    for i in 0..n - 1 {
        input! {
            row: [usize; n - 1 - i],
        }
        for (k, &x) in row.iter().enumerate() {
            let j = i + 1 + k;
            a[i][j] = x;
            a[j][i] = x;
        }
    }

    let mut graph = vec![vec![]; n];

    for i in 1..n {
        let mut parent = INF;
        let mut best_dist = INF;

        for j in 0..n {
            if i == j {
                continue;
            }
            if a[0][j] + a[j][i] == a[0][i] && a[i][j] < best_dist {
                best_dist = a[i][j];
                parent = j;
            }
        }

        if parent == INF {
            println!("No");
            return;
        }

        graph[i].push((parent, a[i][parent]));
        graph[parent].push((i, a[i][parent]));
    }

    for s in 0..n {
        let mut dist = vec![INF; n];
        dist[s] = 0;
        dfs(s, INF, &graph, &mut dist);

        for t in 0..n {
            if dist[t] != a[s][t] {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}

fn dfs(v: usize, p: usize, graph: &[Vec<(usize, usize)>], dist: &mut [usize]) {
    for &(to, w) in &graph[v] {
        if to == p {
            continue;
        }
        dist[to] = dist[v] + w;
        dfs(to, v, graph, dist);
    }
}
