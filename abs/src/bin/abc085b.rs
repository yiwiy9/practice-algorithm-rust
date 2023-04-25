use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: u32,
        mut d: [i32; n],
    }

    let mut set = HashSet::new();
    for d_i in &d {
        set.insert(d_i);
    }

    println!("{}", set.len());
}
