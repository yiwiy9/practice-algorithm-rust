use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    let mut dp = vec![vec![vec![0.0; 101]; 101]; 101];
    for i in (a..=99).rev() {
        for j in (b..=99).rev() {
            for k in (c..=99).rev() {
                let mut sum = 0.0;
                sum += (dp[i + 1][j][k] + 1.0) * i as f64 / (i + j + k) as f64;
                sum += (dp[i][j + 1][k] + 1.0) * j as f64 / (i + j + k) as f64;
                sum += (dp[i][j][k + 1] + 1.0) * k as f64 / (i + j + k) as f64;
                dp[i][j][k] = sum;
            }
        }
    }

    println!("{}", dp[a][b][c]);
}
