use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    let mut pipe_cnt = 0;
    let mut ok = false;
    for &c in &s {
        match c {
            '|' => pipe_cnt += 1,
            '*' => {
                if pipe_cnt == 1 {
                    ok = true;
                }
            }
            _ => {}
        }
    }

    println!("{}", if ok { "in" } else { "out" });
}
