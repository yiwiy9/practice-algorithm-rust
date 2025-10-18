use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; n]; m],
    }

    let mut set = HashSet::new();
    for a_i in &a {
        for j in 0..n - 1 {
            set.insert((a_i[j].min(a_i[j + 1]), a_i[j].max(a_i[j + 1])));
        }
    }

    println!("{}", n * (n - 1) / 2 - set.len());
}
