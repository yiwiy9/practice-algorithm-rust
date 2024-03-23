use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        x: [usize; q],
    }

    a.sort();
    for x_i in x {
        let i = a.lower_bound(&x_i);
        println!("{}", n - i);
    }
}
