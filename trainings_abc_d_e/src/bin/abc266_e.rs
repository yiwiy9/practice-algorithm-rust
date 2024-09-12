use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans: f64 = 0.0;
    for _ in (1..=n).rev() {
        let mut sum = 0.0;
        for num in 1..=6 {
            if num as f64 > ans {
                sum += num as f64;
            } else {
                sum += ans;
            }
        }
        ans = sum / 6.0;
    }

    println!("{:.10}", ans);
}
