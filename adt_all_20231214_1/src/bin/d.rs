use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
    }

    let mut is_prefix = true;
    let mut is_suffix = true;

    for i in 0..n {
        if s[i] != t[i] {
            is_prefix = false;
            break;
        }
    }
    for i in 0..n {
        if s[n - 1 - i] != t[m - 1 - i] {
            is_suffix = false;
            break;
        }
    }

    println!(
        "{}",
        if is_prefix {
            if is_suffix {
                0
            } else {
                1
            }
        } else {
            if is_suffix {
                2
            } else {
                3
            }
        }
    );
}
