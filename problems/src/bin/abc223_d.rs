use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/**
 * https://atcoder.jp/contests/abc223/tasks/abc223_d
 * https://atcoder.jp/contests/abc223/editorial/2777
 *
 * 辞書順最小トポロジカルソート
 * - DFSは使えない
 * - 辞書順最小は前から貪欲
 * - 要素の追加と最小値(最大値)の取得/削除は優先度付きキュー
 *
 */
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    let mut in_degree = vec![0; n];
    for (a, b) in ab {
        graph[a].push(b);
        in_degree[b] += 1;
    }

    let mut heap = BinaryHeap::new();
    for (i, &cnt) in in_degree.iter().enumerate() {
        if cnt == 0 {
            heap.push(Reverse(i));
        }
    }

    let mut order = vec![];
    while let Some(Reverse(i)) = heap.pop() {
        order.push(i);
        for &next_i in &graph[i] {
            in_degree[next_i] -= 1;
            if in_degree[next_i] == 0 {
                heap.push(Reverse(next_i));
            }
        }
    }

    if order.len() != n {
        println!("-1");
    } else {
        println!("{}", order.iter().map(|i| i + 1).join(" "));
    }
}
