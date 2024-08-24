use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut deque = VecDeque::new();
    for _ in 0..q {
        input! {
            op: usize,
        }

        match op {
            1 => {
                input! {
                    x: usize,
                    c: usize,
                }
                deque.push_back((x, c));
            }
            2 => {
                input! {
                    mut c: usize,
                }

                let mut sum = 0;
                while c > 0 {
                    let (x, d) = deque.pop_front().unwrap();
                    if c >= d {
                        sum += d * x;
                        c -= d;
                    } else {
                        sum += c * x;
                        deque.push_front((x, d - c));
                        break;
                    }
                }
                println!("{}", sum);
            }
            _ => unreachable!(),
        }
    }
}
