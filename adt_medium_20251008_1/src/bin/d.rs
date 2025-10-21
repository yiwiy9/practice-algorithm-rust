use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    for i in 1..n {
        let mut l = 0;
        for k in 0..n - i {
            if s[k] != s[k + i] {
                l += 1;
            } else {
                break;
            }
        }
        println!("{}", l);
    }
}
