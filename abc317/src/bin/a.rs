use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        h: usize,
        x: usize,
        p: [usize; n],
    }

    println!("{}", p.lower_bound(&(x - h)) + 1);
}
