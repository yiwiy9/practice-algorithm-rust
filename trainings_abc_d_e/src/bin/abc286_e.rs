use proconio::{
    input,
    marker::{Chars, Usize1},
};

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        s: [Chars; n],
        q: usize,
        uv: [(Usize1, Usize1); q],
    }

    let mut dp = vec![vec![(INF, 0); n]; n];
    for i in 0..n {
        dp[i][i] = (0, a[i]);
    }
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'Y' {
                dp[i][j] = (1, a[i] + a[j]);
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let cost = (dp[i][k].0 + dp[k][j].0, dp[i][k].1 + dp[k][j].1 - a[k]);
                if cost.0 < dp[i][j].0 || (cost.0 == dp[i][j].0 && cost.1 > dp[i][j].1) {
                    dp[i][j] = cost;
                }
            }
        }
    }

    for &(u, v) in &uv {
        if dp[u][v].0 == INF {
            println!("Impossible");
        } else {
            println!("{} {}", dp[u][v].0, dp[u][v].1);
        }
    }
}
