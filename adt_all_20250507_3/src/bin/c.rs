use proconio::{input, marker::Chars};

fn main() {
    input! {
        s1: Chars,
        s2: Chars,
        s3: Chars,
        t: Chars,
    }

    for &c in &t {
        if c == '1' {
            print!("{}", s1.iter().collect::<String>());
        } else if c == '2' {
            print!("{}", s2.iter().collect::<String>());
        } else if c == '3' {
            print!("{}", s3.iter().collect::<String>());
        }
    }
    println!();
}
