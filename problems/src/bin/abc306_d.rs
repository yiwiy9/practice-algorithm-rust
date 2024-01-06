use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize,i64); n],
    }

    let mut dp = vec![vec![0; 2]; n + 1];
    for i in 0..n {
        dp[i + 1][0] = dp[i + 1][0].max(dp[i][0]);
        dp[i + 1][1] = dp[i + 1][1].max(dp[i][1]);
        match xy[i].0 {
            0 => {
                dp[i + 1][0] = dp[i + 1][0].max(dp[i][0] + xy[i].1);
                dp[i + 1][0] = dp[i + 1][0].max(dp[i][1] + xy[i].1);
            }
            1 => {
                dp[i + 1][1] = dp[i + 1][1].max(dp[i][0] + xy[i].1);
            }
            _ => unreachable!(),
        }
    }

    println!("{}", dp[n][0].max(dp[n][1]));
}
