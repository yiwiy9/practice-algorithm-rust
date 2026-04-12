use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut ok = false;
    for &c in &s {
        if c != 'o' {
            ok = true;
            print!("{}", c);
        } else if ok {
            print!("{}", c);
        }
    }

    println!("");
}
