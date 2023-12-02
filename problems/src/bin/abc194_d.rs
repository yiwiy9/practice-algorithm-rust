use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut dp = vec![0.0; n + 1];
    dp[n] = 0.0;
    for i in (0..n).rev() {
        dp[i] = dp[i + 1] + i as f64 / (n - i) as f64 + 1.0;
    }

    println!("{:.8}", dp[1]);
}
