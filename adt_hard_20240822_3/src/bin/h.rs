use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        x: f64,
        y: f64,
    }

    println!("{:.10}", rec(a, x, y, n, &mut HashMap::new()));
}

fn rec(a: usize, x: f64, y: f64, n: usize, memo: &mut HashMap<usize, f64>) -> f64 {
    if n == 0 {
        return 0.0;
    }
    if let Some(&result) = memo.get(&n) {
        return result;
    }

    let x_rec = x + rec(a, x, y, n / a, memo);

    let mut y_rec = y * 6.0;
    for i in 2..=6 {
        y_rec += rec(a, x, y, n / i, memo);
    }
    y_rec /= 5.0;

    let ans = x_rec.min(y_rec);

    memo.insert(n, ans);
    ans
}
