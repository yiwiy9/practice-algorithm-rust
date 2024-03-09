use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        mut k: usize,
        s: Chars,
    }

    for c in s {
        if c == 'o' && k > 0 {
            k -= 1;
            print!("o");
            continue;
        }
        print!("x");
    }

    println!();
}
