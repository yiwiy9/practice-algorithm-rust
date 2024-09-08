use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    }

    let n = s.len();
    let mut ans = vec![];
    for i in 0..n {
        if s[i] > t[i] {
            s[i] = t[i];
            ans.push(s.clone());
        }
    }

    for i in (0..n).rev() {
        if s[i] != t[i] {
            s[i] = t[i];
            ans.push(s.clone());
        }
    }

    println!("{}", ans.len());
    if ans.is_empty() {
        return;
    }
    println!(
        "{}",
        ans.iter()
            .map(|s| s.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
}
