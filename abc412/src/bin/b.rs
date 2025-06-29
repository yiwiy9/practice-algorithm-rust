use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    for i in 1..s.len() {
        if s[i].is_uppercase() && !t.contains(&s[i - 1]) {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
