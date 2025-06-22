use proconio::{input, marker::Chars};

fn main() {
    input! {
        p: Chars,
        l: usize,
    }

    println!("{}", if p.len() >= l { "Yes" } else { "No" });
}
