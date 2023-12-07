use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        mut t: Chars,
    }

    s.sort();
    t.sort_by(|a, b| b.cmp(a));

    let mut has_changed = false;
    let mut ok: bool = false;
    for i in 0..(s.len().min(t.len())) {
        if s[i] < t[i] {
            ok = true;
            has_changed = true;
            break;
        } else if s[i] > t[i] {
            has_changed = true;
            break;
        }
    }

    if !has_changed {
        ok = s.len() < t.len();
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
