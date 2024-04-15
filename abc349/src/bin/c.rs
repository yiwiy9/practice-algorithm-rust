use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut j = 0;
    for &c in &s {
        if c.to_ascii_uppercase() == t[j] {
            j += 1;
            if j == 3 {
                break;
            }
        }
    }

    if j == 2 && t[j] == 'X' {
        j += 1;
    }

    println!("{}", if j == 3 { "Yes" } else { "No" });
}
