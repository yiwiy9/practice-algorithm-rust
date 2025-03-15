use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    if s.iter().all(|&s_i| s_i == 'o') || s.iter().all(|&s_i| s_i == '-') {
        println!("-1");
        return;
    }

    let mut ans = 0;
    let mut cnt = 0;
    for &s_i in &s {
        if s_i == 'o' {
            cnt += 1;
        } else {
            cnt = 0;
        }
        ans = ans.max(cnt);
    }

    println!("{}", ans);
}
