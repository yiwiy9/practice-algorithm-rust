use ac_library::MfGraph;
use proconio::input;
use std::collections::{BTreeMap, BTreeSet};

/// https://atcoder.jp/contests/abc434/tasks/abc434_e
/// https://atcoder.jp/contests/abc434/editorial/14684
/// => ネットワークフロー問題に帰着する
///    最大流問題、最小カット問題 => けんちょん本: フォード・ファルカーソン法 O(F|E|) ← 遅い
///
/// それぞれのうさぎの始点から行き先候補の点に有向辺を張る
/// そうすると、辺の集合であって複数の辺に覆われた頂点が存在しないものの最大サイズを求める
/// 最大マッチング のサイズを計算する問題になる
///
/// 二部グラフの最大マッチング（二部マッチング）は最大流を用いて解くことができる
/// （各辺の容量は 1 とする）
///
/// ACL の最大流ライブラリを使用すれば、O(N√N) で解くことができて、今回の制約でも間に合う
/// https://atcoder.github.io/ac-library/production/document_ja/maxflow.html
/// https://github.com/rust-lang-ja/ac-library-rs/blob/master/examples/practice2_d_maxflow.rs
fn main() {
    input! {
        n: usize,
        xr: [(i64, i64); n],
    }

    let mut v_set = BTreeSet::new();
    for &(x, r) in &xr {
        let v_minus = x - r;
        let v_plus = x + r;
        v_set.insert(v_minus);
        v_set.insert(v_plus);
    }

    let mut v_idx_map = BTreeMap::new();
    let mut v_idx = n + 1;
    for &v in &v_set {
        v_idx_map.insert(v, v_idx);
        v_idx += 1;
    }

    // 最大流を流す始点と終点となる頂点を用意する
    let source = v_idx;
    let sink = source + 1;

    // n頂点, 0辺のグラフを作る
    let mut graph = MfGraph::<usize>::new(sink + 1);

    // 始点からそれぞれのうさぎに対して、容量1の有向辺を張る
    for i in 0..n {
        graph.add_edge(source, i, 1);
    }

    // それぞれのうさぎから行き先候補に対して、容量1の有向辺を張る
    for (i, &(x, r)) in xr.iter().enumerate() {
        let v_minus_idx = *v_idx_map.get(&(x - r)).unwrap();
        let v_plus_idx = *v_idx_map.get(&(x + r)).unwrap();
        graph.add_edge(i, v_minus_idx, 1);
        graph.add_edge(i, v_plus_idx, 1);
    }

    // それぞれのうさぎの行き先候補から終点に対して、容量1の有向辺を張る
    for j in n + 1..v_idx {
        graph.add_edge(j, sink, 1);
    }

    println!("{}", graph.flow(source, sink));
}

// use proconio::input;
// use std::collections::{BTreeMap, BTreeSet};

// => ポイントはグラフの問題として捉え直す
//
// グラフ問題として捉えると、
// 各辺について隣接する頂点のどちらかに色を塗ります。(同じ頂点に複数回色を塗っても良い) 色を 1 回以上塗られた頂点の個数を最大化してください。
//
// 1. グラフが木である (すなわち n=m+1 である) 場合
//    => m = n - 1 より、答えは n - 1
//
// 2. グラフが木でない (すなわち m≥n である) 場合
//    => 連結成分に含まれる頂点の個数は n 個なので頂点は高々 n 個しか塗ることが出来ません。
//    => また、以下の手順で n 個塗ることが必ず達成できます。
//       1. G の全域木を 1 個取り T とする。
//       2. G に含まれて T に含まれない辺が必ず存在するが、そのうち 1 本を取る。その辺は u−v 間を結ぶとする。
//       3. T を頂点 v を根とする根付き木とみなして、各辺について子の側の頂点を塗る。グラフの頂点は頂点 v 以外が塗られた状態になる。
//       4. 先に選んだ u−v 辺を用いて頂点 v を塗る。
// fn main() {
//     input! {
//         n: usize,
//         xr: [(i64, i64); n],
//     }

//     let mut graph_map = BTreeMap::new();
//     for (i, &(x, r)) in xr.iter().enumerate() {
//         let u = x - r;
//         let v = x + r;
//         graph_map.entry(u).or_insert(vec![]).push((v, i));
//         graph_map.entry(v).or_insert(vec![]).push((u, i));
//     }

//     let mut ans = 0;
//     let mut seen = BTreeSet::new();
//     for &u in graph_map.keys() {
//         if seen.contains(&u) {
//             continue;
//         }
//         let (cnt, is_tree) = dfs(&graph_map, &mut seen, u, n);
//         ans += if is_tree { cnt - 1 } else { cnt };
//     }

//     println!("{}", ans);
// }

// fn dfs(
//     graph_map: &BTreeMap<i64, Vec<(i64, usize)>>,
//     seen: &mut BTreeSet<i64>,
//     v: i64,
//     parent_edge: usize,
// ) -> (usize, bool) {
//     seen.insert(v);

//     let mut cnt = 1;
//     let mut is_tree = true;
//     for &(next_v, edge) in &graph_map[&v] {
//         if edge == parent_edge {
//             continue;
//         }

//         if seen.contains(&next_v) {
//             is_tree = false;
//             continue;
//         }

//         let (child_cnt, child_is_tree) = dfs(graph_map, seen, next_v, edge);
//         cnt += child_cnt;
//         is_tree &= child_is_tree;
//     }

//     (cnt, is_tree)
// }
