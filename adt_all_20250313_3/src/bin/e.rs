use proconio::{input, marker::Usize1};
use superslice::*;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        ab: [(Usize1,Usize1); n],
    }

    let mut row_vec = vec![];
    let mut col_vec = vec![];
    for &(a_i, b_i) in &ab {
        row_vec.push(a_i);
        col_vec.push(b_i);
    }
    row_vec.sort();
    row_vec.dedup();
    col_vec.sort();
    col_vec.dedup();

    for &(a_i, b_i) in &ab {
        let row_i = row_vec.lower_bound(&a_i);
        let col_i = col_vec.lower_bound(&b_i);
        println!("{} {}", row_i + 1, col_i + 1);
    }
}
