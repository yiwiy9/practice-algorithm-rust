use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut ok = false;
    for c in 1..s.len() {
        for w in c..s.len() {
            let mut u = vec![];
            let mut idx = 0;
            while idx < s.len() {
                for i in 0..w {
                    if idx == s.len() {
                        break;
                    }

                    if i + 1 == c {
                        u.push(s[idx]);
                    }

                    idx += 1;
                }
            }

            let mut cur_ok = true;
            if u.len() != t.len() {
                cur_ok = false;
            }
            if !cur_ok {
                continue;
            }
            for i in 0..u.len() {
                if u[i] != t[i] {
                    cur_ok = false;
                    break;
                }
            }

            if cur_ok {
                ok = true;
                break;
            }
        }
        if ok {
            break;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
