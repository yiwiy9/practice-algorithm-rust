use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars
    }

    println!("{}", s.last().unwrap());
}
