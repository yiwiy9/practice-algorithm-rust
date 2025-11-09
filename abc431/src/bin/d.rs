use proconio::input;

fn main() {
    input! {
        n: usize,
        whb: [(usize,usize,usize); n],
    }

    let w_sum = whb.iter().map(|&(w, _, _)| w).sum::<usize>();
    let w_half = w_sum / 2;

    let mut dp = vec![0; w_half + 1];
    for &(w, h, b) in &whb {
        let mut next_dp = vec![0; w_half + 1];
        for j in 0..=w_half {
            if j + w <= w_half {
                next_dp[j + w] = next_dp[j + w].max(dp[j] + h);
            }
            next_dp[j] = next_dp[j].max(dp[j] + b);
        }
        dp = next_dp;
    }

    println!("{}", dp.iter().max().unwrap());
}
