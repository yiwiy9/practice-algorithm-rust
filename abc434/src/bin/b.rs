use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize,f64); n],
    }

    for i in 1..=m {
        let mut sum = 0.0;
        let mut cnt = 0;
        for &(a, b) in &ab {
            if a == i {
                sum += b;
                cnt += 1;
            }
        }
        println!("{:.10}", sum / (cnt as f64));
    }
}
