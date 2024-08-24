use proconio::{input, marker::Chars};
const MAX: usize = 1_000_000_000_000_000_000;

fn main() {
    input! {
        _: usize,
        mut x: usize,
        s: Chars,
    }

    let mut over_cnt = 0;
    for &c in &s {
        if over_cnt > 0 {
            match c {
                'U' => {
                    over_cnt -= 1;
                }
                'L' => {
                    over_cnt += 1;
                }
                'R' => {
                    over_cnt += 1;
                }
                _ => unreachable!(),
            }

            continue;
        }

        match c {
            'U' => {
                x /= 2;
            }
            'L' => {
                if 2 * x <= MAX {
                    x *= 2;
                } else {
                    over_cnt += 1;
                }
            }
            'R' => {
                if 2 * x + 1 <= MAX {
                    x = 2 * x + 1;
                } else {
                    over_cnt += 1;
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{}", x);
}
