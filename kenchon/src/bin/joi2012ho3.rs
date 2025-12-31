use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        s: usize,
        ab: [(i64,usize); n],
    }

    let mut dp = vec![vec![-1; 2]; t + 1];
    dp[0][0] = 0;
    for &(a, b) in &ab {
        let mut next_dp = vec![vec![-1; 2]; t + 1];
        for j in 0..=t {
            for k in 0..2 {
                if dp[j][k] == -1 {
                    continue;
                }

                next_dp[j][k] = next_dp[j][k].max(dp[j][k]);

                if j + b <= t {
                    next_dp[j + b][k] = next_dp[j + b][k].max(dp[j][k] + a);
                }

                if k == 0 && j <= s {
                    next_dp[s][1] = next_dp[s][1].max(next_dp[j][0]);
                }
            }
        }
        dp = next_dp;
    }

    println!("{}", dp.iter().map(|v| v[1]).max().unwrap());
}
