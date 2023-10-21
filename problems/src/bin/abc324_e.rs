use proconio::{input, marker::Chars};
use superslice::*;

fn main() {
    input! {
        n: usize,
        t: Chars,
        s: [Chars; n],
    }

    let t_last_idx = t.len() as i32 - 1;

    let mut left_vec = vec![];
    let mut right_vec = vec![];
    for s_i in &s {
        let mut left_t_idx: i32 = -1;
        for &c in s_i {
            if left_t_idx < t_last_idx && c == t[(left_t_idx + 1) as usize] {
                left_t_idx += 1;
            }
        }
        left_vec.push(left_t_idx);

        let mut right_t_idx = t_last_idx + 1;
        for &c in s_i.iter().rev() {
            if right_t_idx > 0 && c == t[(right_t_idx - 1) as usize] {
                right_t_idx -= 1;
            }
        }
        right_vec.push(right_t_idx);
    }

    right_vec.sort();

    let mut ans = 0;
    for left_t_idx in left_vec {
        ans += right_vec.upper_bound(&(left_t_idx + 1));
    }

    println!("{}", ans);
}
