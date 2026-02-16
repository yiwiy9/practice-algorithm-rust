use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    println!(
        "{}",
        if s[0] == *s.last().unwrap() {
            "Yes"
        } else {
            "No"
        }
    );
}
