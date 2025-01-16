use proconio::{input, marker::Usize1};

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, usize); m],
    }

    let mut dp = vec![vec![INF; n]; n];
    for &(a, b, c) in &abc {
        dp[a][b] = dp[a][b].min(c);
    }
    for i in 0..n {
        dp[i][i] = 0;
    }

    let mut ans = 0;
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j]);
                if dp[i][j] != INF {
                    ans += dp[i][j];
                }
            }
        }
    }

    println!("{}", ans);
}
