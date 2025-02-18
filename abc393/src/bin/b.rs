use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0;
    for k in 1..50 {
        for i in 0..s.len() {
            if i + k + k >= s.len() {
                break;
            }
            if s[i] == 'A' && s[i + k] == 'B' && s[i + k + k] == 'C' {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
