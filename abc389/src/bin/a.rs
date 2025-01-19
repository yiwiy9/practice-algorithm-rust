use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    println!("{}", (s[0] as u8 - b'0') * (s[2] as u8 - b'0'));
}
