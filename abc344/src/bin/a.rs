use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut flg = false;
    let mut cnt = 0;
    for &c in &s {
        if c == '|' {
            cnt += 1;
        }

        if cnt % 2 == 0 && c != '|' {
            flg = true;
            print!("{}", c);
        }
    }

    if flg {
        println!();
    }
}
