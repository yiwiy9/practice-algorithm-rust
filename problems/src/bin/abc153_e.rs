use proconio::input;
const INF: usize = 1 << 30;

fn main() {
    input! {
        h: usize,
        n: usize,
        ab: [(usize,usize); n],
    }

    let mut dp = vec![vec![INF; h + 1]; n + 1];
    dp[0][0] = 0;

    for i in 0..n {
        for j in 0..=h {
            if dp[i][j] != INF {
                dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);
            }
            if dp[i + 1][j] != INF {
                if j + ab[i].0 < h {
                    dp[i + 1][j + ab[i].0] = dp[i + 1][j + ab[i].0].min(dp[i + 1][j] + ab[i].1);
                } else {
                    dp[i + 1][h] = dp[i + 1][h].min(dp[i + 1][j] + ab[i].1);
                }
            }
        }
    }

    println!("{}", dp[n][h]);
}
