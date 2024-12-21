use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    for i in 0..n - 2 {
        let a = s[i];
        let b = s[i + 1];
        let c = s[i + 2];
        if a == 'A' && b == 'B' && c == 'C' {
            println!("{}", i + 1);
            return;
        }
    }

    println!("-1");
}
