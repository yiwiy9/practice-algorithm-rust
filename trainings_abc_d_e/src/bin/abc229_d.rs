use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        s: Chars,
        k: usize,
    }
    let mut mut_k = k;

    let mut queue = VecDeque::new();
    let mut x_continuos_cnt = 0;
    let mut cur = 0;
    let mut ans = 0;
    for c in s {
        match c {
            'X' => {
                x_continuos_cnt += 1;
                cur += 1;
                ans = ans.max(cur);
            }
            '.' => {
                if k == 0 {
                    cur = 0;
                    x_continuos_cnt = 0;
                    continue;
                }

                if mut_k != 0 {
                    mut_k -= 1;
                } else {
                    cur -= queue.pop_front().unwrap();
                }
                queue.push_back(x_continuos_cnt + 1);
                cur += 1;
                ans = ans.max(cur);
                x_continuos_cnt = 0;
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ans);
}
