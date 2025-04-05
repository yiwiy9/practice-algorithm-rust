use proconio::{input, marker::Usize1};
use std::collections::HashSet;
use superslice::*;

fn main() {
    input! {
        n: usize,
        q: usize,
        x: usize,
        w: [usize; n],
        k: [Usize1; q],
    }

    let mut s_w = vec![0; n + 1];
    for i in 0..n {
        s_w[i + 1] = s_w[i] + w[i];
    }

    let mut start_set = HashSet::new();
    let mut start_vec = vec![];
    let mut i = 0;
    while !start_set.contains(&i) {
        start_set.insert(i);

        let loop_cnt = x / s_w[n];
        let mut cnt = loop_cnt * n;

        let mut remain = x - loop_cnt * s_w[n];
        let mut lower_bound = s_w.lower_bound(&(remain + s_w[i]));
        cnt += lower_bound - i;

        if lower_bound == n + 1 {
            remain -= s_w[n] - s_w[i];
            lower_bound = s_w.lower_bound(&(remain));
            cnt += lower_bound - 1;
        }

        start_vec.push((i, cnt));
        i = lower_bound;
    }

    let loop_start = start_vec.iter().position(|&(x, _)| x == i).unwrap();
    let loop_len = start_vec.len() - loop_start;

    for &k_i in &k {
        if k_i < loop_start {
            println!("{}", start_vec[k_i].1);
            continue;
        }

        let k_i = k_i - loop_start;
        let loop_k = k_i % loop_len;
        println!("{}", start_vec[loop_start + loop_k].1);
    }
}
