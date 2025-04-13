use proconio::input;

const MOD: usize = 1_000_000_000;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut ans = vec![0; n + 1];
    let mut sum = 0;
    for i in 0..=n {
        if i < k {
            ans[i] = 1;
        } else {
            ans[i] = sum;
            sum += MOD - ans[i - k];
            sum %= MOD;
        }

        sum += ans[i];
        sum %= MOD;
    }

    println!("{}", ans[n]);
}
