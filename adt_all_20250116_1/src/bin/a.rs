use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut b = vec![0; n - 1];
    for i in 0..n - 1 {
        b[i] = a[i + 1] * a[i];
    }

    println!("{}", b.iter().join(" "));
}
