use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut ok = false;
    for i in 0..s.len() {
        let mut s_new = s.clone();
        if i > 0 {
            s_new.swap(i, i - 1);
        }

        if (0..s.len()).all(|j| s_new[j] == t[j]) {
            ok = true;
            break;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
