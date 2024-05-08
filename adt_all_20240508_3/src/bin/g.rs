use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut r: [usize; n],
        query: [usize; q],
    }

    r.sort();
    let mut sum_r = vec![0; n + 1];
    for i in 0..n {
        sum_r[i + 1] = sum_r[i] + r[i];
    }

    for &q in &query {
        println!("{}", sum_r.upper_bound(&q) - 1);
    }
}
