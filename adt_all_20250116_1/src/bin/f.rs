use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    if s.iter().all(|&c| c == 'o') || s.iter().all(|&c| c == '-') {
        println!("-1");
        return;
    }

    let mut ans = -1;
    let mut cur = 0;
    for i in 0..n {
        if s[i] == 'o' {
            cur += 1;
        } else {
            ans = ans.max(cur);
            cur = 0;
        }
    }
    ans = ans.max(cur);

    println!("{}", ans);
}
