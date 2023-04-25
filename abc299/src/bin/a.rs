use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    let mut ok = false;
    let mut cnt = 0;
    for &c in &s {
        match c {
            '*' => {
                if cnt == 1 {
                    ok = true;
                }
            }
            '|' => cnt += 1,
            _ => continue,
        }
    }

    println!("{}", if ok { "in" } else { "out" });
}
