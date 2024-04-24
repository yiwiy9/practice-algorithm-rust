use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    for i in 1..n {
        let mut max_l = 0;
        for l in 1..=n - i {
            if s[l - 1] != s[l + i - 1] {
                max_l = l;
            } else {
                break;
            }
        }
        println!("{}", max_l);
    }
}
