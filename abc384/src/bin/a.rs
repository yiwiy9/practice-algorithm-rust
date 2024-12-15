use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        c1: char,
        c2: char,
        s: Chars,
    }

    let mut ans = vec![];
    for i in 0..n {
        if s[i] != c1 {
            ans.push(c2);
        } else {
            ans.push(s[i]);
        }
    }

    println!("{}", ans.iter().collect::<String>());
}
