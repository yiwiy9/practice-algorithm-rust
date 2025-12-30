use proconio::input;

const MOD: usize = 1234567;

fn main() {
    input! {
        n: usize,
        p: usize,
        h: [usize; n],
    }

    let mut dp = vec![0; n + 1];

    let mut cur_h_sum = 0;
    let mut left_h_sum = 0;

    dp[0] = 1;
    let mut dp_sum = 1;
    for i in 1..=n {
        cur_h_sum += h[i - 1];

        while cur_h_sum > p {
            cur_h_sum -= h[left_h_sum];

            dp_sum += MOD - dp[left_h_sum];
            dp_sum %= MOD;

            left_h_sum += 1;
        }

        dp[i] = dp_sum;

        dp_sum += dp[i];
        dp_sum %= MOD;
    }

    println!("{}", dp[n]);
}
