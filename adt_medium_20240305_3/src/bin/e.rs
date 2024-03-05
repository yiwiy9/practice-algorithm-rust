use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    for i in 1..=n {
        let next_idx = a.lower_bound(&i);
        println!("{}", a[next_idx] - i);
    }
}
