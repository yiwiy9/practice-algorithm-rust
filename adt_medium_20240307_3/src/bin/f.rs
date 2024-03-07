use proconio::input;
use superslice::*;

const INF: i64 = 1 << 60;

fn main() {
    input! {
        d: i64,
    }

    let square_vec = (0..=2_000_000).map(|i| i * i).collect::<Vec<i64>>();

    let mut ans = INF;
    for &x in &square_vec {
        if x >= d {
            break;
        }

        let yy = d - x;
        let yy_lower_i = square_vec.lower_bound(&yy);

        ans = ans.min((x + square_vec[yy_lower_i] - d).abs());
        if yy_lower_i > 0 {
            ans = ans.min((x + square_vec[yy_lower_i - 1] - d).abs());
        }
    }

    println!("{}", ans);
}
