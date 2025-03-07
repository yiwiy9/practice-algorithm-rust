use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0;
    for i in 0..10 {
        if !s.contains(&char::from_digit(i, 10).unwrap()) {
            ans = i;
            break;
        }
    }

    println!("{}", ans);
}
