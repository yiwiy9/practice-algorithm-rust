use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    }

    let mut ans = vec![];
    let mut i = 0;
    while i < s.len() {
        if s[i] > t[i] {
            s[i] = t[i];
            ans.push(s.clone());
        }
        i += 1;
    }

    for i in (0..s.len()).rev() {
        if s[i] != t[i] {
            s[i] = t[i];
            ans.push(s.clone());
        }
    }

    println!("{}", ans.len());
    if !ans.is_empty() {
        println!(
            "{}",
            ans.iter()
                .map(|x| x.iter().collect::<String>())
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}
