use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::BTreeSet;
use superslice::*;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        queries: [(Usize1, Usize1); q],
    }

    let mut one_count = vec![0; n + 1];
    let mut two_count = vec![0; n + 1];
    let mut set = BTreeSet::new();
    for i in 0..n {
        one_count[i + 1] = one_count[i] + if s[i] == '1' { 1 } else { 0 };
        two_count[i + 1] = two_count[i] + if s[i] == '2' { 1 } else { 0 };
        if s[i] == '/' {
            set.insert(i);
        }
    }

    for &(l, r) in &queries {
        let left_slash = *set.range(l..).next().unwrap_or(&n);
        let right_slash = *set.range(..=r).next_back().unwrap_or(&0);
        if left_slash > right_slash {
            println!("0");
            continue;
        }

        let mut left = 0;
        let mut right = r - l + 1;
        while right - left > 1 {
            let mid = (left + right) / 2;

            let one_right = one_count.lower_bound(&(mid + one_count[l]));
            let slash_min = *set.range(one_right..).next().unwrap_or(&n);

            if slash_min == n {
                right = mid;
                continue;
            }

            let two_min = two_count.lower_bound(&(mid + two_count[slash_min + 1]));

            if two_min <= r + 1 {
                left = mid;
            } else {
                right = mid;
            }
        }

        println!("{}", left * 2 + 1);
    }
}
