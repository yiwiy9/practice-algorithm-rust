use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut dp = vec![vec![0_usize; 2]; n + 1];
    dp[0][0] = 1;
    dp[0][1] = 1;

    for i in 1..=n {
        if s[i - 1] == "AND" {
            // xi == 1
            dp[i][1] += dp[i - 1][1];
            dp[i][0] += dp[i - 1][0];
            // xi == 0
            dp[i][0] += dp[i - 1][1];
            dp[i][0] += dp[i - 1][0];
        } else {
            // xi == 1
            dp[i][1] += dp[i - 1][1];
            dp[i][1] += dp[i - 1][0];
            // xi == 0
            dp[i][1] += dp[i - 1][1];
            dp[i][0] += dp[i - 1][0];
        }
    }

    println!("{}", dp[n][1]);
}
