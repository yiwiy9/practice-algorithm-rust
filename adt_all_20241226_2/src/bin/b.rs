use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    println!(
        "{}",
        if s.iter().any(|&c| c == 'o') && s.iter().all(|&c| c != 'x') {
            "Yes"
        } else {
            "No"
        }
    );
}
