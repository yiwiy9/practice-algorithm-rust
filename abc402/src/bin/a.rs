use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = vec![];
    for &c in &s {
        if c.is_ascii_uppercase() {
            ans.push(c);
        }
    }

    println!("{}", ans.iter().collect::<String>());
}
