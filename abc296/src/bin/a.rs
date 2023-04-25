use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: i32,
        s: Chars,
    }

    let mut ok = true;
    for i in 1..s.len() {
        if s[i] == s[i - 1] {
            ok = false;
            break;
        }
    }

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
