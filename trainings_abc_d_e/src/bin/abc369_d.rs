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
            dp[i + 1][j ^ 1] =
                dp[i + 1][j ^ 1].max(dp[i][j] + a[i] + if j ^ 1 == 0 { a[i] } else { 0 });
        }
    }

    println!("{}", dp[n][0].max(dp[n][1]));
}
