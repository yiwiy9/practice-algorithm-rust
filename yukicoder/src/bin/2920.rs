use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut a_cnt = 0;
    let mut b_cnt = 0;
    let mut ab_cnt = 0;
    let mut o_cnt = 0;

    for i in 0..2 {
        for j in 0..2 {
            let a = if s[i] == 'A' { 1 } else { 0 } + if t[j] == 'A' { 1 } else { 0 };
            let b = if s[i] == 'B' { 1 } else { 0 } + if t[j] == 'B' { 1 } else { 0 };

            if a == 1 && b == 1 {
                ab_cnt += 1;
            } else if a >= 1 && b == 0 {
                a_cnt += 1;
            } else if a == 0 && b >= 1 {
                b_cnt += 1;
            } else {
                o_cnt += 1;
            }
        }
    }

    println!(
        "{} {} {} {}",
        a_cnt * 100 / 4,
        b_cnt * 100 / 4,
        ab_cnt * 100 / 4,
        o_cnt * 100 / 4
    );
}
