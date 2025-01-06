use proconio::{input, marker::Chars};

const INF: usize = 1 << 60;

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
        s: Chars,
    }

    let mut dp = vec![INF; 2];
    dp[0] = 0;
    for &c in &s {
        let mut next_dp = vec![INF; 2];

        if c == 'a' {
            next_dp[0] = next_dp[0].min(dp[0] + x);
            next_dp[1] = next_dp[1].min(dp[0] + z + y);

            next_dp[1] = next_dp[1].min(dp[1] + y);
            next_dp[0] = next_dp[0].min(dp[1] + z + x);
        } else {
            next_dp[1] = next_dp[1].min(dp[1] + x);
            next_dp[0] = next_dp[0].min(dp[1] + z + y);

            next_dp[0] = next_dp[0].min(dp[0] + y);
            next_dp[1] = next_dp[1].min(dp[0] + z + x);
        }

        dp = next_dp;
    }

    println!("{}", dp[0].min(dp[1]));
}
