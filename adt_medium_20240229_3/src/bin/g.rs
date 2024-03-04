use proconio::{input, marker::Chars};

const INF: usize = 1 << 60;

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
        s: Chars,
    }

    let mut dp = vec![vec![INF; 2]; s.len() + 1];
    dp[0][0] = 0;
    for (i, &c) in s.iter().enumerate() {
        if c == 'A' {
            dp[i + 1][0] = dp[i + 1][0].min(dp[i][0] + y);
            dp[i + 1][0] = dp[i + 1][0].min(dp[i][1] + z + y);
            dp[i + 1][1] = dp[i + 1][1].min(dp[i][0] + z + x);
            dp[i + 1][1] = dp[i + 1][1].min(dp[i][1] + x);
        } else {
            dp[i + 1][0] = dp[i + 1][0].min(dp[i][0] + x);
            dp[i + 1][0] = dp[i + 1][0].min(dp[i][1] + z + x);
            dp[i + 1][1] = dp[i + 1][1].min(dp[i][0] + z + y);
            dp[i + 1][1] = dp[i + 1][1].min(dp[i][1] + y);
        }
    }

    println!("{}", dp[s.len()][0].min(dp[s.len()][1]));
}
