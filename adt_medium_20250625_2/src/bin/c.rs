use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; k],
        xy: [(f64,f64); n],
    }

    let mut max_len: f64 = 0.0;
    for &(x, y) in &xy {
        let mut min_len: f64 = 100_000_000_000.0;
        for &i in &a {
            min_len = min_len.min((x - xy[i].0) * (x - xy[i].0) + (y - xy[i].1) * (y - xy[i].1));
        }
        max_len = max_len.max(min_len);
    }

    println!("{:.10}", max_len.sqrt());
}
