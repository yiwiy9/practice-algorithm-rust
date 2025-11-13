use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        t: Chars,
        a: Chars,
    }

    println!(
        "{}",
        if (0..n).any(|i| t[i] == 'o' && t[i] == a[i]) {
            "Yes"
        } else {
            "No"
        }
    );
}
