use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        mut a: [usize; 3],
    }

    a.sort_by(|a, b| b.cmp(a));

    println!("{}", a.iter().join(""));
}
