use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut n: usize,
        s: Chars,
    }

    let mut ans: usize = 0;
    let mut right_count = 0;
    let mut before_c = '_';
    for &c in &s {
        if before_c != '_' && c != before_c {
            ans += right_count * (n - right_count);
            n -= right_count;
            right_count = 0;
        }
        right_count += 1;
        before_c = c;
    }

    println!("{}", ans);
}
