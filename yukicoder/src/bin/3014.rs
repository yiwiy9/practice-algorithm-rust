use proconio::input;
const INF: i64 = 1 << 60;

fn main() {
    input! {
        n: usize,
        d: usize,
        k: usize,
        a: [i64; n],
        c: [usize; n],
    }

    let mut dp = vec![vec![-INF; k + 1]; d + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let mut next_dp = vec![vec![-INF; k + 1]; d + 1];
        for j in 0..=d {
            for l in 0..=k {
                if dp[j][l] == -INF {
                    continue;
                }
                next_dp[j][l] = next_dp[j][l].max(dp[j][l]);
                if j < d {
                    next_dp[j + 1][(l + c[i]).min(k)] =
                        next_dp[j + 1][(l + c[i]).min(k)].max(dp[j][l] + a[i]);
                }
            }
        }
        dp = next_dp;
    }
    if dp[d][k] == -INF {
        println!("No");
    } else {
        println!("{}", dp[d][k]);
    }
}
