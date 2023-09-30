use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    for i in 0..n {
        println!("{}", a[a.lower_bound(&(i + 1))] - (i + 1));
    }
}
