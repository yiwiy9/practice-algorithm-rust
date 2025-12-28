use proconio::input;

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        d: usize,
        a: [[usize; n]; d],
    }

    let mut dp = vec![vec![INF; 3]; n];
    for i in 0..n {
        dp[i][0] = a[0][i];
    }

    for j in 1..d {
        let mut next_dp = vec![vec![INF; 3]; n];
        let min_before = *dp
            .iter()
            .map(|dp_i| dp_i.iter().min().unwrap())
            .min()
            .unwrap();
        for i in 0..n {
            next_dp[i][0] = next_dp[i][0].min(min_before + a[j][i]);

            if dp[i][0] != INF {
                next_dp[i][1] = next_dp[i][1].min(dp[i][0] + a[j][i] * 9 / 10);
            }
            if dp[i][1] != INF {
                next_dp[i][2] = next_dp[i][2].min(dp[i][1] + a[j][i] * 7 / 10);
            }
            if dp[i][2] != INF {
                next_dp[i][2] = next_dp[i][2].min(dp[i][2] + a[j][i] * 7 / 10);
            }
        }
        dp = next_dp;
    }

    println!(
        "{}",
        dp.iter()
            .map(|dp_i| dp_i.iter().min().unwrap())
            .min()
            .unwrap()
    );
}
