use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut t = vec![];
    let mut i = 0;
    while i < s.len() {
        if i != s.len() - 1 && s[i] == 'i' && s[i + 1] == 'o' {
            i += 2;
        } else {
            t.push(s[i]);
            i += 1;
        }
    }

    println!("{}", t.len());
}
