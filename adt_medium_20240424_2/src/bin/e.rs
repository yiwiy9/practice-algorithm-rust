use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [(usize,usize,usize); n],
    }

    let p_sum = p.iter().map(|(a, b, c)| a + b + c).collect::<Vec<_>>();

    let mut p_sum_sorted = p_sum.clone();
    p_sum_sorted.sort();

    for sum in p_sum {
        let grade = n - p_sum_sorted.upper_bound(&(sum + 300)) + 1;
        println!("{}", if grade <= k { "Yes" } else { "No" });
    }
}
