use proconio::input;
use std::collections::{BTreeMap, BTreeSet};

/// https://atcoder.jp/contests/abc434/tasks/abc434_e
/// https://atcoder.jp/contests/abc434/editorial/14684
/// => ポイントはグラフの問題として捉え直す
///
/// グラフ問題として捉えると、
/// 各辺について隣接する頂点のどちらかに色を塗ります。(同じ頂点に複数回色を塗っても良い) 色を 1 回以上塗られた頂点の個数を最大化してください。
///
/// 1. グラフが木である (すなわち n=m+1 である) 場合
///    => m = n - 1 より、答えは n - 1
///
/// 2. グラフが木でない (すなわち m≥n である) 場合
///    => 連結成分に含まれる頂点の個数は n 個なので頂点は高々 n 個しか塗ることが出来ません。
///    => また、以下の手順で n 個塗ることが必ず達成できます。
///       1. G の全域木を 1 個取り T とする。
///       2. G に含まれて T に含まれない辺が必ず存在するが、そのうち 1 本を取る。その辺は u−v 間を結ぶとする。
///       3. T を頂点 v を根とする根付き木とみなして、各辺について子の側の頂点を塗る。グラフの頂点は頂点 v 以外が塗られた状態になる。
///       4. 先に選んだ u−v 辺を用いて頂点 v を塗る。
fn main() {
    input! {
        n: usize,
        xr: [(i64, i64); n],
    }

    let mut graph_map = BTreeMap::new();
    for (i, &(x, r)) in xr.iter().enumerate() {
        let u = x - r;
        let v = x + r;
        graph_map.entry(u).or_insert(vec![]).push((v, i));
        graph_map.entry(v).or_insert(vec![]).push((u, i));
    }

    let mut ans = 0;
    let mut seen = BTreeSet::new();
    for &u in graph_map.keys() {
        if seen.contains(&u) {
            continue;
        }
        let (cnt, is_tree) = dfs(&graph_map, &mut seen, u, n);
        ans += if is_tree { cnt - 1 } else { cnt };
    }

    println!("{}", ans);
}

fn dfs(
    graph_map: &BTreeMap<i64, Vec<(i64, usize)>>,
    seen: &mut BTreeSet<i64>,
    v: i64,
    parent_edge: usize,
) -> (usize, bool) {
    seen.insert(v);

    let mut cnt = 1;
    let mut is_tree = true;
    for &(next_v, edge) in &graph_map[&v] {
        if edge == parent_edge {
            continue;
        }

        if seen.contains(&next_v) {
            is_tree = false;
            continue;
        }

        let (child_cnt, child_is_tree) = dfs(graph_map, seen, next_v, edge);
        cnt += child_cnt;
        is_tree &= child_is_tree;
    }

    (cnt, is_tree)
}
