use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
        c: [i64; n],
    }

    let abc = vec![a, b, c];

    let mut dp = vec![vec![-1; 3]; n];
    dp[0][0] = abc[0][0];
    for i in 1..n {
        for j in 0..3 {
            if dp[i - 1][j] != -1 {
                dp[i][j] = dp[i][j].max(dp[i - 1][j] + abc[j][i]);
            }

            if j > 0 && dp[i - 1][j - 1] != -1 {
                dp[i][j] = dp[i][j].max(dp[i - 1][j - 1] + abc[j][i]);
            }
        }
    }

    println!("{}", dp.iter().map(|v| v[2]).max().unwrap());
}
