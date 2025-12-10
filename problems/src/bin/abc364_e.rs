use proconio::input;

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        ab: [(usize,usize); n],
    }

    let mut dp = vec![vec![INF; n + 1]; x + 1];
    dp[0][0] = 0;

    for &(a, b) in &ab {
        // やっぱり、next_dpが大事（i-1 と i を同時に持つと、iの更新中にiの更新結果を使うことになる）
        let mut next_dp = vec![vec![INF; n + 1]; x + 1];
        for x_sum in 0..=x {
            for cnt in 0..=n {
                if dp[x_sum][cnt] == INF {
                    continue;
                }

                next_dp[x_sum][cnt] = next_dp[x_sum][cnt].min(dp[x_sum][cnt]);

                let next_x_sum = x_sum + a;
                if next_x_sum <= x && cnt < n {
                    let next_y_sum = dp[x_sum][cnt] + b;
                    if next_y_sum <= y {
                        next_dp[next_x_sum][cnt + 1] = next_dp[next_x_sum][cnt + 1].min(next_y_sum);
                    }
                }
            }
        }
        dp = next_dp;
    }

    let mut ans = 0;
    for x_sum in 0..=x {
        for cnt in 0..=n {
            if dp[x_sum][cnt] == INF {
                continue;
            }
            ans = ans.max(cnt);
        }
    }

    println!("{}", (ans + 1).min(n));
}
