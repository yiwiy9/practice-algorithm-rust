use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars
    }

    for i in 0..n.len() {
        print!("{}", if i < 3 { n[i] } else { '0' });
    }

    println!();
}
