use proconio::{input, marker::Usize1};

const MOD: i64 = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        s: Usize1,
        t: Usize1,
        x: Usize1,
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for (u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut memo = vec![vec![vec![-1; 2]; k + 1]; n];
    let ans = dfs(&graph, t, x, &mut memo, k, 1, s);

    println!("{}", ans);
}

fn dfs(
    graph: &Vec<Vec<usize>>,
    t: usize,
    x: usize,
    memo: &mut Vec<Vec<Vec<i64>>>,
    k: usize,
    is_even: usize,
    cur: usize,
) -> i64 {
    if memo[cur][k][is_even] != -1 {
        return memo[cur][k][is_even];
    }

    let mut res = 0;
    if k == 0 {
        if cur == t && is_even == 1 {
            res = 1;
        }
    } else {
        for &next in &graph[cur] {
            let next_is_even = ((is_even == 1) ^ (next == x)) as usize;
            res += dfs(graph, t, x, memo, k - 1, next_is_even, next);
            res %= MOD;
        }
    }

    memo[cur][k][is_even] = res;
    res
}
