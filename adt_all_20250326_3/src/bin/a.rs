use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        d: usize,
        s: Chars,
    }

    println!("{}", d + s.iter().filter(|&&c| c == '.').count());
}
