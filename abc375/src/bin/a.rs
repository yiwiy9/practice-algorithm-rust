use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut ans = 0;
    for i in 2..n {
        if s[i - 2] == '#' && s[i - 1] == '.' && s[i] == '#' {
            ans += 1;
        }
    }

    println!("{}", ans);
}
