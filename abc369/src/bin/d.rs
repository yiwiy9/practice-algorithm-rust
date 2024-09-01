use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut dp = vec![vec![-1; 2]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..2 {
            if dp[i][j] == -1 {
                continue;
            }

            dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);

            if j == 0 {
                dp[i + 1][1] = dp[i + 1][1].max(dp[i][j] + a[i]);
            } else {
                dp[i + 1][0] = dp[i + 1][0].max(dp[i][j] + 2 * a[i]);
            }
        }
    }

    println!("{}", dp[n][0].max(dp[n][1]));
}
