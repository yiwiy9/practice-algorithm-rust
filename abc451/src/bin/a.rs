use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    println!("{}", if s.len() % 5 == 0 { "Yes" } else { "No" });
}
