use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
    }

    let mut is_prefix = true;
    for i in 0..n {
        if s[i] != t[i] {
            is_prefix = false;
        }
    }

    let mut is_suffix = true;
    for i in (0..n).rev() {
        if s[i] != t[m - n + i] {
            is_suffix = false;
        }
    }

    println!(
        "{}",
        if is_prefix && is_suffix {
            0
        } else if is_prefix {
            1
        } else if is_suffix {
            2
        } else {
            3
        }
    );
}
