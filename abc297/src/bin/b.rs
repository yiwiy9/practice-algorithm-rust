use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut before_b_pos = 9;
    let mut seen_r_cnt = 0;
    let mut ok = true;

    for (i, c) in s.iter().enumerate() {
        if *c == 'B' {
            if before_b_pos == 9 {
                before_b_pos = i;
            } else if before_b_pos % 2 == i % 2 {
                ok = false;
                break;
            }
        }
        if *c == 'R' {
            seen_r_cnt += 1;
        }
        if *c == 'K' && seen_r_cnt != 1 {
            ok = false;
            break;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
