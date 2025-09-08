use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut first = s[0] as usize - '0' as usize;
    let mut second = s[2] as usize - '0' as usize;

    if second == 8 {
        first += 1;
        second = 1;
    } else {
        second += 1;
    }

    println!("{}-{}", first, second);
}
