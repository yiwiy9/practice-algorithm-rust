use proconio::{input, marker::Chars};

const INF: usize = 1 << 60;

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
        s: Chars,
    }
    let n = s.len();

    let mut dp = vec![vec![INF; 2]; n + 1];
    dp[0][0] = 0;

    for i in 1..=n {
        if s[i - 1] == 'a' {
            dp[i][0] = dp[i][0].min(dp[i - 1][0] + x);
            dp[i][0] = dp[i][0].min(dp[i - 1][1] + z + x);

            dp[i][1] = dp[i][1].min(dp[i - 1][1] + y);
            dp[i][1] = dp[i][1].min(dp[i - 1][0] + z + y);
        } else {
            dp[i][0] = dp[i][0].min(dp[i - 1][0] + y);
            dp[i][0] = dp[i][0].min(dp[i - 1][1] + z + y);

            dp[i][1] = dp[i][1].min(dp[i - 1][1] + x);
            dp[i][1] = dp[i][1].min(dp[i - 1][0] + z + x);
        }
    }

    println!("{}", dp[n][0].min(dp[n][1]));
}
