use proconio::{input, marker::Usize1};
const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        m: usize,
        uvt: [(Usize1, Usize1, usize); m],
        q: usize,
    }

    let mut dp = vec![vec![INF; n]; n];
    for &(u, v, t) in &uvt {
        dp[u][v] = dp[u][v].min(t);
        dp[v][u] = dp[v][u].min(t);
    }
    for i in 0..n {
        dp[i][i] = 0;
    }

    // ワーシャルフロイド法
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j]);
            }
        }
    }

    for _ in 0..q {
        input! {
            k: usize,
            mut b: [Usize1; k],
        }

        let mut ans = INF;

        // next_permutation()
        permutohedron::heap_recursive(&mut b, |b| {
            ans = ans.min(rec(&mut dp, n, &uvt, k, b, 0, 0));
        });

        println!("{}", ans);
    }
}

fn rec(
    dp: &mut Vec<Vec<usize>>,
    n: usize,
    uvt: &[(usize, usize, usize)],
    k: usize,
    b: &[usize],
    i: usize,
    v: usize,
) -> usize {
    if i == k {
        return dp[v][n - 1];
    }

    let mut ans = INF;

    ans = ans.min(dp[v][uvt[b[i]].0] + uvt[b[i]].2 + rec(dp, n, uvt, k, b, i + 1, uvt[b[i]].1));

    ans = ans.min(dp[v][uvt[b[i]].1] + uvt[b[i]].2 + rec(dp, n, uvt, k, b, i + 1, uvt[b[i]].0));

    ans
}
