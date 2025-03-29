use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        n: usize,
    }

    let mut set = HashSet::new();
    for _ in 0..n {
        input! {
            l: usize,
            a: [usize; l],
        }
        set.insert(a);
    }

    println!("{}", set.len());
}
