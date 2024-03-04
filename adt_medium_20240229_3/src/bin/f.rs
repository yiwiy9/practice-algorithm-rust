use itertools::Itertools;
use proconio::input;
use superslice::*;

fn main() {
    input! {
        _: usize,
        _: usize,
        n: usize,
        ab: [(usize,usize); n],
    }

    let mut a = ab.iter().map(|&(a, _)| a).unique().collect::<Vec<_>>();
    let mut b = ab.iter().map(|&(_, b)| b).unique().collect::<Vec<_>>();

    a.sort();
    b.sort();

    for &(a_i, b_i) in &ab {
        let a_idx = a.lower_bound(&a_i) + 1;
        let b_idx = b.lower_bound(&b_i) + 1;
        println!("{} {}", a_idx, b_idx);
    }
}
