use itertools::Itertools;
use proconio::input;

fn main() {
    let mut b = vec![];
    loop {
        input! {
            a: usize,
        }
        b.push(a);
        if a == 0 {
            break;
        }
    }

    println!("{}", b.iter().rev().join("\n"));
}
