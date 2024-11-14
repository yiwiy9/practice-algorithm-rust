use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let mut ans = 0;
    let mut cnt = 0;
    for &c in &s {
        if c == 'X' {
            cnt = 0;
        } else {
            cnt += 1;
            if cnt >= k {
                ans += 1;
                cnt = 0;
            }
        }
    }

    println!("{}", ans);
}
