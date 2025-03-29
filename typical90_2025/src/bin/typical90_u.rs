use ac_library::SccGraph;
use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/typical90/tasks/typical90_u
/// https://x.com/e869120/status/1385363292739104775
///
/// 強連結成分分解 - SCC
/// 有向グラフにおいて「お互いに行き来できる <=> 同じグループ」を満たすように頂点をグループ分けすることを強連結成分分解と言います。
/// https://manabitimes.jp/math/1250
///
/// ACLibrary の強連結成分分解を使うと、O(N + M) で強連結成分分解ができる。
/// https://atcoder.github.io/ac-library/production/document_ja/scc.html
fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = SccGraph::new(n);
    for &(u, v) in &uv {
        graph.add_edge(u, v);
    }

    let scc = graph.scc();

    let mut ans = 0;
    for v_vec in &scc {
        // 例: [0, 1, 2, 3]
        //     3→2,1,0, 2→1,0, 1→0
        //     3 + 2 + 1 = 6
        // よって、 1~(n-1)の和 = n(n-1)/2 が頂点リストごとの答え
        ans += v_vec.len() * (v_vec.len() - 1) / 2;
    }

    println!("{}", ans);
}
