use proconio::input;
use superslice::*;

fn main() {
    input! {
        d: i64,
    }

    let mut square_vec = vec![];
    for i in 0..=((d as f64).sqrt() as i64) {
        square_vec.push(i * i);
    }

    let mut ans = d;
    for &x in &square_vec {
        let y_pos = square_vec.lower_bound(&(d - x));
        if y_pos < square_vec.len() {
            ans = ans.min((x + square_vec[y_pos] - d).abs());
        }
        if y_pos > 0 {
            ans = ans.min((x + square_vec[y_pos - 1] - d).abs());
        }
    }

    println!("{}", ans);
}
