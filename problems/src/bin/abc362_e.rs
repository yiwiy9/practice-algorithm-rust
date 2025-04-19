use ac_library::modint::ModInt998244353 as Mint;
use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut graph = vec![vec![]; n];
    for u in 0..n {
        for v in u + 1..n {
            graph[u].push((v, a[v] - a[u]));
        }
    }

    let mut memo = HashMap::new();
    let mut ans = vec![Mint::new(0); n];
    for u in 0..n {
        ans[0] += 1;

        for &(next_v, next_sub) in &graph[u] {
            let res = dfs(&graph, &mut memo, next_sub, next_v);
            for i in 0..res.len() {
                ans[i] += res[i];
            }
        }
    }

    println!("{}", ans.iter().join(" "));
}

pub fn dfs(
    graph: &Vec<Vec<(usize, i64)>>,
    memo: &mut HashMap<(usize, i64), Vec<Mint>>,
    sub: i64,
    v: usize,
) -> Vec<Mint> {
    if memo.contains_key(&(v, sub)) {
        return memo[&(v, sub)].clone();
    }

    let mut cur = vec![Mint::new(0); graph.len()];
    for &(next_v, next_sub) in &graph[v] {
        if next_sub != sub {
            continue;
        }
        let res = dfs(graph, memo, next_sub, next_v);
        for i in 0..res.len() {
            if i + 1 < cur.len() {
                cur[i + 1] += res[i];
            }
        }
    }

    cur[1] += 1;
    memo.insert((v, sub), cur.clone());

    cur
}
