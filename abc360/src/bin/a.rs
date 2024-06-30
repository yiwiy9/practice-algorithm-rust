use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut r_i = 0;
    let mut m_i = 0;
    for i in 0..s.len() {
        if s[i] == 'R' {
            r_i = i;
        }
        if s[i] == 'M' {
            m_i = i;
        }
    }

    println!("{}", if r_i < m_i { "Yes" } else { "No" });
}
