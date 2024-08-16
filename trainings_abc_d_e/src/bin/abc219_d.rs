use proconio::input;
const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        ab: [(usize, usize); n],
    }

    let mut dp = vec![vec![vec![INF; y + 1]; x + 1]; n + 1];
    dp[0][0][0] = 0;

    for i in 0..n {
        let (a, b) = ab[i];
        for j in 0..=x {
            for k in 0..=y {
                dp[i + 1][j][k] = dp[i + 1][j][k].min(dp[i][j][k]);
                dp[i + 1][(j + a).min(x)][(k + b).min(y)] =
                    dp[i + 1][(j + a).min(x)][(k + b).min(y)].min(dp[i][j][k] + 1);
            }
        }
    }

    println!(
        "{}",
        if dp[n][x][y] == INF {
            -1
        } else {
            dp[n][x][y] as i64
        }
    );
}
