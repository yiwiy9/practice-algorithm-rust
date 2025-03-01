use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let a = s[0].to_digit(10).unwrap();
    let b = s[1].to_digit(10).unwrap();
    let c = s[2].to_digit(10).unwrap();

    println!(
        "{}",
        100 * a + 10 * a + a + 100 * b + 10 * b + b + 100 * c + 10 * c + c
    );
}
