use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(f64,f64); n],
    }

    xy.push((0.0, 0.0));

    let mut ans = 0.0;
    let mut cur = (0.0, 0.0);
    for (x, y) in xy {
        let (cx, cy) = cur;
        ans += ((x - cx) * (x - cx) + (y - cy) * (y - cy)).sqrt();
        cur = (x, y);
    }

    println!("{:.10}", ans);
}
