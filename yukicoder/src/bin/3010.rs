use proconio::{input,marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut ans = 0;
    for _ in 0..n {
        input! {
            s: Chars,
            r: usize,
        }
        if r < 1200 {
            continue;
        }
        if s.iter().take(4).any(|&c| c == 'x') {
            ans += 1;
        }
    }

    println!("{}", ans);
}
