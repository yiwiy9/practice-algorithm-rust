use proconio::{input, marker::Chars};

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        s: Chars,
        c: [usize; n],
    }

    let mut dp = vec![vec![vec![INF; 2]; 2]; n + 1];
    dp[0][0][0] = 0;
    dp[0][1][0] = 0;
    for i in 0..n {
        if s[i] == '0' {
            // cur: 0

            // before: 1
            dp[i + 1][0][0] = dp[i + 1][0][0].min(dp[i][1][0]);
            dp[i + 1][0][1] = dp[i + 1][0][1].min(dp[i][1][1]);
            if i == 0 {
                dp[i + 1][1][0] = dp[i + 1][1][0].min(dp[i][1][0] + c[i]);
            } else {
                dp[i + 1][1][1] = dp[i + 1][1][1].min(dp[i][1][0] + c[i]);
            }
            // before: 0
            dp[i + 1][1][0] = dp[i + 1][1][0].min(dp[i][0][0] + c[i]);
            dp[i + 1][1][1] = dp[i + 1][1][1].min(dp[i][0][1] + c[i]);
            if i == 0 {
                dp[i + 1][0][0] = dp[i + 1][0][0].min(dp[i][0][0]);
            } else {
                dp[i + 1][0][1] = dp[i + 1][0][1].min(dp[i][0][0]);
            }
        } else {
            //cur: 1

            // before: 1
            dp[i + 1][0][0] = dp[i + 1][0][0].min(dp[i][1][0] + c[i]);
            dp[i + 1][0][1] = dp[i + 1][0][1].min(dp[i][1][1] + c[i]);
            if i == 0 {
                dp[i + 1][1][0] = dp[i + 1][1][0].min(dp[i][1][0]);
            } else {
                dp[i + 1][1][1] = dp[i + 1][1][1].min(dp[i][1][0]);
            }

            // before: 0
            dp[i + 1][1][0] = dp[i + 1][1][0].min(dp[i][0][0]);
            dp[i + 1][1][1] = dp[i + 1][1][1].min(dp[i][0][1]);
            if i == 0 {
                dp[i + 1][0][0] = dp[i + 1][0][0].min(dp[i][0][0] + c[i]);
            } else {
                dp[i + 1][0][1] = dp[i + 1][0][1].min(dp[i][0][0] + c[i]);
            }
        }
    }

    println!("{}", dp[n][0][1].min(dp[n][1][1]));
}
