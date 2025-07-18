use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();
    a.dedup();

    println!("{}", a.len());
    println!("{}", a.iter().join(" "));
}
