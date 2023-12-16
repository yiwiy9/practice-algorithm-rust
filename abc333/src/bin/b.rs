use std::mem::swap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut s_len = ((s[0] as i32 - 'A' as i32) - (s[1] as i32 - 'A' as i32)).abs();
    let mut t_len = ((t[0] as i32 - 'A' as i32) - (t[1] as i32 - 'A' as i32)).abs();

    if s_len > t_len {
        swap(&mut s_len, &mut t_len);
    }
    println!(
        "{}",
        if (s_len == t_len) || (s_len == 2 && t_len == 3) || (s_len == 1 && t_len == 4) {
            "Yes"
        } else {
            "No"
        }
    );
}
