use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n],
    }

    let mut ans: f64 = 0.0;
    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1) = xy[i];
            let (x2, y2) = xy[j];
            ans = ans.max(((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt());
        }
    }

    println!("{:.10}", ans);
}
