use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        d: usize,
        s: [Chars; n],
    }

    let mut ans = 0;
    let mut cur = 0;
    for j in 0..d {
        let mut ok = true;
        for i in 0..n {
            if s[i][j] == 'x' {
                ok = false;
            }
        }
        if ok {
            cur += 1;
            ans = ans.max(cur);
        } else {
            cur = 0;
        }
    }

    println!("{}", ans);
}
