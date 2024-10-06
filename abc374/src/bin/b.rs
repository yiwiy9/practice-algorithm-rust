use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let n = s.len().max(t.len());
    for i in 0..n {
        if i < s.len() && i < t.len() && s[i] == t[i] {
            continue;
        }
        println!("{}", i + 1);
        return;
    }

    println!("{}", 0);
}
