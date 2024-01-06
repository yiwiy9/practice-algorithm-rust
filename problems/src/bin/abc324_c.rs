use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::mem::swap;

fn main() {
    input! {
        n: usize,
        mut t: Chars,
        mut s: [Chars; n],
    }

    let mut ans = vec![];
    for (i, s_i) in s.iter_mut().enumerate() {
        let mut is_swapped = false;
        if s_i.len() > t.len() {
            swap(s_i, &mut t);
            is_swapped = true;
        }

        if t.len() - s_i.len() > 1 {
            if is_swapped {
                swap(s_i, &mut t);
            }
            continue;
        }

        let mut miss_cnt = 0;
        let mut j = 0;
        let mut k = 0;
        loop {
            if miss_cnt > 1 || j == s_i.len() {
                break;
            }
            if s_i[j] != t[k] {
                miss_cnt += 1;
                if s_i.len() != t.len() {
                    k += 1;
                    continue;
                }
            }
            j += 1;
            k += 1;
        }

        if miss_cnt <= 1 {
            ans.push(i + 1);
        }

        if is_swapped {
            swap(s_i, &mut t);
        }
    }

    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
