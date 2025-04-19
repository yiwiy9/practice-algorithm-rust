use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: usize,
        x: f64,
        y: f64,
    }

    println!("{:.10}", rec(a, x, y, &mut HashMap::new(), n));
}

fn rec(a: usize, x: f64, y: f64, memo: &mut HashMap<usize, f64>, i: usize) -> f64 {
    if let Some(&v) = memo.get(&i) {
        return v;
    }
    if i == 0 {
        return 0.0;
    }

    let a_val = rec(a, x, y, memo, i / a) + x;

    let mut b_val = 0.0;
    for b in 2..=6 {
        b_val += rec(a, x, y, memo, i / b) / 5.0;
    }
    b_val += y * 6.0 / 5.0;

    let ans = a_val.min(b_val);
    memo.insert(i, ans);
    ans
}
