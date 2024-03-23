use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    let mut enclosed = false;
    for &c in &s {
        if c == '"' {
            enclosed = !enclosed;
            print!("{}", c);
            continue;
        }

        if c == ',' && !enclosed {
            print!(".");
            continue;
        }

        print!("{}", c);
    }

    println!();
}
