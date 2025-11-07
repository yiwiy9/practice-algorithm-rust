use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut queue = VecDeque::new();
    for (i, &c) in s.iter().enumerate() {
        if c == '/' {
            queue.push_back(i);
        }
    }

    let mut ans = 0;
    while let Some(slash_idx) = queue.pop_front() {
        let mut left_one_cnt = 0;
        for i in (0..slash_idx).rev() {
            if s[i] == '1' {
                left_one_cnt += 1;
            } else {
                break;
            }
        }
        let mut right_two_cnt = 0;
        for i in slash_idx + 1..n {
            if s[i] == '2' {
                right_two_cnt += 1;
            } else {
                break;
            }
        }

        ans = ans.max(left_one_cnt.min(right_two_cnt) * 2 + 1);
    }

    println!("{}", ans);
}
