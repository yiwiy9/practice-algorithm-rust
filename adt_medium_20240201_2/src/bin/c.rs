use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();

    let mut ans = 0;
    for i in 0..n {
        for j in (i..n).rev() {
            let mut ok = true;
            for c in 0..=((j - i) / 2) {
                if s[i + c] != s[j - c] {
                    ok = false;
                    break;
                }
            }
            if ok {
                ans = ans.max(j - i + 1);
            }
        }
    }

    println!("{}", ans);
}
