use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
    }

    let mut cnt = 0;
    let mut t_sum = 0;
    let mut queue = VecDeque::new();
    for _ in 0..q {
        input! {
            op: usize,
        }

        match op {
            1 => {
                cnt += 1;
            }
            2 => {
                input! {
                    t: usize,
                }
                queue.push_back((t_sum, cnt));
                t_sum += t;
                cnt = 0;
            }
            3 => {
                input! {
                    h: usize,
                }
                let mut ans = 0;
                while let Some(&(t, c)) = queue.front() {
                    if h > t_sum - t {
                        break;
                    }
                    ans += c;
                    queue.pop_front();
                }
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}
