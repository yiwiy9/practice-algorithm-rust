use proconio::input;

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        ab: [(usize,usize); n],
    }

    let mut dp = vec![vec![INF; y + 1]; x + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let (a, b) = ab[i];
        let mut next_dp = dp.clone();
        for j in (0..=x).rev() {
            for k in (0..=y).rev() {
                if dp[j][k] == INF {
                    continue;
                }
                let next_j = (j + a).min(x);
                let next_k = (k + b).min(y);
                next_dp[next_j][next_k] = next_dp[next_j][next_k].min(dp[j][k] + 1);
            }
        }
        dp = next_dp;
    }

    println!("{}", if dp[x][y] == INF { -1 } else { dp[x][y] as i64 });
}
