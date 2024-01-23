use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut ok = true;
    let mut cur = s[0];

    for &c in &s {
        if c == cur {
            continue;
        }
        if cur == 'A' {
            cur = c;
            continue;
        }
        if cur == 'B' {
            if c == 'C' {
                cur = c;
                continue;
            } else {
                ok = false;
                break;
            }
        }
        if cur == 'C' {
            ok = false;
            break;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
