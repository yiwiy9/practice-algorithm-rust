use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    for (i, &c) in s.iter().enumerate() {
        if c.is_ascii_uppercase() {
            println!("{}", i + 1);
        }
    }
}
