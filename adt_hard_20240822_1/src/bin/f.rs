use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(f64, f64); n],
    }

    let all_len = ab.iter().map(|&(a, _)| a).sum::<f64>();
    let all_time = ab.iter().map(|&(a, b)| a / b).sum::<f64>();

    let mut left = 0.0;
    let mut right = all_len + 1.0;
    for _ in 0..100 {
        let mid = (left + right) / 2.0;

        let mut left_time = 0.0;
        let mut left_cur = 0.0;
        for &(a, b) in &ab {
            if left_cur + a > mid {
                left_time += (mid - left_cur) / b;
                break;
            }
            left_time += a / b;
            left_cur += a;
        }

        if left_time * 2.0 <= all_time {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{:.10}", left);
}
