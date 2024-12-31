use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        mut s: Chars,
        mut t: Chars,
    }

    if s.len() > t.len() {
        std::mem::swap(&mut s, &mut t);
    }

    if t.len() - s.len() > 1 {
        println!("No");
        return;
    }

    if t.len() == s.len() {
        let mut diff = 0;
        for i in 0..s.len() {
            if s[i] != t[i] {
                diff += 1;
            }
        }
        println!("{}", if diff <= 1 { "Yes" } else { "No" });
        return;
    }

    let mut diff = 0;
    let mut i = 0;
    let mut j = 0;
    while i < s.len() && j < t.len() {
        if s[i] != t[j] {
            diff += 1;
            j += 1;
        } else {
            i += 1;
            j += 1;
        }
    }

    println!("{}", if diff <= 1 { "Yes" } else { "No" });
}
