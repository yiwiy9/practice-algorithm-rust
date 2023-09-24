use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }

    let mut ok = true;
    for (i, &n_i) in n.iter().enumerate().skip(1) {
        if (n[i - 1] as i32) <= n_i as i32 {
            ok = false;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
