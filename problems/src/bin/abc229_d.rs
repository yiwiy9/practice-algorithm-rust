use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        s: Chars,
        mut k: usize,
    }

    let mut ans = 0;
    let mut start_i = 0;
    let mut que = VecDeque::new();

    for (i, s_i) in s.iter().enumerate() {
        match *s_i {
            'X' => {
                ans = ans.max(i + 1 - start_i);
            }
            '.' => {
                if k == 0 {
                    que.push_back(i);
                    start_i = que.pop_front().unwrap() + 1;
                } else {
                    que.push_back(i);
                    k -= 1;
                    ans = ans.max(i + 1 - start_i);
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ans);
}
