use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    println!(
        "{}",
        if s[s.len() - 3] == 's' && s[s.len() - 2] == 'a' && s[s.len() - 1] == 'n' {
            "Yes"
        } else {
            "No"
        }
    );
}
