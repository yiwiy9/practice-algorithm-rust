use ac_library::Dsu;
use proconio::{input, marker::Usize1};
use std::collections::BTreeMap;

/**
 * https://atcoder.jp/contests/abc350/tasks/abc350_d
 * https://atcoder.jp/contests/abc350/editorial/9811
 * 操作は、すでに同じ連結成分に属する頂点の間に辺を追加するものであるため、連結成分が変化することはありません。
 * 逆に同じ連結成分に属する頂点同士は、操作の繰り返しにより必ず直接辺で結ぶことができます。
 *
 * したがって、連結成分ごとに完全グラフとした場合の辺の数（nC2 = n(n-1)/2）を求め、それらの和から与えられた辺の数を引いたものが答え
 */
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1,Usize1); m],
    }

    let mut dsu = Dsu::new(n);
    for (a, b) in ab {
        dsu.merge(a, b);
    }

    let mut cnt_map = BTreeMap::new();
    for i in 0..n {
        cnt_map
            .entry(dsu.leader(i))
            .and_modify(|cur| *cur += 1)
            .or_insert(1);
    }

    let mut ans = 0;
    for (_, &v) in &cnt_map {
        ans += v * (v - 1) / 2;
    }
    ans -= m;

    println!("{}", ans);
}
