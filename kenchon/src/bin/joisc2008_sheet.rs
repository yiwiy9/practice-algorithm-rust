use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;

/// https://atcoder.jp/contests/joisc2008/tasks/joisc2008_sheet
/// - 何ごとの話にすると見やすい？ 何だけ持てばいい？
///   最終写真から分かる色紙同士の上下関係の話にして、各色紙について「自分より上にある必要がある色紙」だけ持てばよい。
/// - それについて、何が分かれば答えになる？
///   各色紙間の上下制約が分かれば、その制約を満たす置く順番を 1 つ出せばよい。
/// - 何を捨ててよく、なぜそれで足りる？ 何が効く / 何が禁止？
///   色紙の正確な位置や大きさは不要で、写真上で同じ色が見えている最小長方形の内部に別の紙の色が見えるなら、その紙は上に置かれたと分かる。
///   机の色 `0` は紙ではないので、順序制約には入れない。
/// - その情報をどう更新 / 判定 / 集計すれば実装できる？
///   各色の min/max 行列位置を求め、各 bounding box 内の `1..=N` の色へ有向辺を張り、トポロジカルソート順を出力する。
///
/// AC: 45分
fn main() {
    input! {
        n: usize,
        w: usize,
        h: usize,
        c: [[usize; w]; h],
    }

    let mut corners = vec![(h, 0, w, 0); n + 1];
    for (i, row) in c.iter().enumerate() {
        for (j, &c_ij) in row.iter().enumerate() {
            corners[c_ij] = (
                corners[c_ij].0.min(i),
                corners[c_ij].1.max(i),
                corners[c_ij].2.min(j),
                corners[c_ij].3.max(j),
            );
        }
    }

    let mut set_graph = vec![HashSet::new(); n + 1];
    for x in 1..=n {
        for i in corners[x].0..=corners[x].1 {
            for j in corners[x].2..=corners[x].3 {
                set_graph[x].insert(c[i][j]);
            }
        }
    }

    let graph = set_graph
        .iter()
        .map(|set| set.iter().copied().collect_vec())
        .collect_vec();

    let mut seen = vec![false; n + 1];
    let mut order = vec![];
    for v in 1..=n {
        if seen[v] {
            continue;
        }
        dfs(&graph, &mut seen, &mut order, v);
    }
    order.reverse();

    println!("{}", order.iter().join(" "));
}

pub fn dfs(graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, order: &mut Vec<usize>, v: usize) {
    seen[v] = true;

    for &next_v in &graph[v] {
        if seen[next_v] {
            continue;
        }
        dfs(graph, seen, order, next_v);
    }

    // トポロジカルソート順
    order.push(v);
}
