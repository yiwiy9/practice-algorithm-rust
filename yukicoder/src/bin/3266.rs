use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut a: usize,
        s: Chars,
    }

    let mut cur = 1200_i64;
    let mut one_cnt = 0;
    for (i, &c) in s.iter().enumerate() {
        if c == '1' {
            one_cnt += 1;
            if cur < 1200 {
                cur += 1;
                a -= 1;
            }
        } else {
            cur -= 1;
        }

        if a == 0 {
            println!("{}", i + 1);
            return;
        }
    }

    if n - one_cnt >= one_cnt {
        let loop_cnt = (a - 1) / one_cnt;
        a -= loop_cnt * one_cnt;

        for (i, &c) in s.iter().enumerate() {
            if c == '1' {
                a -= 1;
            }
            if a == 0 {
                println!("{}", n * (loop_cnt + 1) + i + 1);
                return;
            }
        }
    } else {
        let loop_cnt = (a - 1) / (n - one_cnt);
        a -= loop_cnt * (n - one_cnt);

        for (i, &c) in s.iter().enumerate() {
            if c == '1' {
                one_cnt += 1;
                if cur < 1200 {
                    cur += 1;
                    a -= 1;
                }
            } else {
                cur -= 1;
            }

            if a == 0 {
                println!("{}", n * (loop_cnt + 1) + i + 1);
                return;
            }
        }
    }
}
