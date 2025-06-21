use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        t: Chars,
        a: Chars,
    }

    let mut ok = false;
    for i in 0..n {
        if t[i] == 'o' && a[i] == 'o' {
            ok = true;
            break;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
