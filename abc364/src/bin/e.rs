use proconio::input;

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        ab: [(usize,usize); n],
    }

    let mut dp = vec![vec![vec![INF; x + 1]; 81]; n + 1];
    dp[0][0][0] = 0;
    for i in 0..n {
        for j in 0..81 {
            for k in 0..(x + 1) {
                if dp[i][j][k] == INF {
                    continue;
                }

                dp[i + 1][j][k] = dp[i + 1][j][k].min(dp[i][j][k]);
                let (a, b) = ab[i];
                if j + 1 <= 80 && k + a <= x && dp[i][j][k] + b <= y {
                    dp[i + 1][j + 1][k + a] = dp[i + 1][j + 1][k + a].min(dp[i][j][k] + b);
                }
            }
        }
    }

    let mut ans = 0;
    for j in 0..81 {
        for k in 0..(x + 1) {
            if dp[n][j][k] != INF {
                ans = ans.max(j);
            }
        }
    }

    println!("{}", n.min(ans + 1));
}
