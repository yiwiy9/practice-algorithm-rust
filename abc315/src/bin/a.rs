use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let ans: String = s
        .iter()
        .filter(|&&c| c != 'a' && c != 'e' && c != 'i' && c != 'o' && c != 'u')
        .collect();
    println!("{}", ans);
}
