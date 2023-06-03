use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }

    for (i, c) in n.iter().enumerate() {
        if i < 3 {
            print!("{}", c);
        } else {
            print!("{}", 0);
        }
    }
    println!()
}
