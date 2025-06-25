use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0;
    for i in 0..s.len() {
        if ((i + ans) % 2 == 0 && s[i] == 'i') || ((i + ans) % 2 == 1 && s[i] == 'o') {
            continue;
        }
        ans += 1;
    }

    if (s.len() + ans) % 2 != 0 {
        ans += 1;
    }

    println!("{}", ans);
}
