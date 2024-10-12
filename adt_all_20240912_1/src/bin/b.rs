use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut ok = false;
    let mut separator_cnt = 0;
    for &c in &s {
        if c == '|' {
            separator_cnt += 1;
        } else if c == '*' && separator_cnt == 1 {
            ok = true;
            break;
        }
    }

    println!("{}", if ok { "in" } else { "out" });
}
