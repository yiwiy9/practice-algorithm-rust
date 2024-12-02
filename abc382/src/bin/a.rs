use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        d: usize,
        s: Chars,
    }

    println!("{}", d + s.iter().filter(|&&c| c == '.').count());
}
