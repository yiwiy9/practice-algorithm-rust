use itertools::Itertools;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        q: usize,
        xh: [(i64, i64); n],
        t: [i64; q],
    }

    let mut xh_sum = xh.iter().map(|&(x, h)| (x + h, x, h)).collect_vec();
    xh_sum.sort();

    let mut left_h_max = vec![0; n];
    left_h_max[0] = xh_sum[0].2;
    for i in 1..n {
        left_h_max[i] = left_h_max[i - 1].max(xh_sum[i].2);
    }

    let mut right_x_min = vec![1 << 60; n];
    right_x_min[n - 1] = xh_sum[n - 1].1;
    for i in (0..n - 1).rev() {
        right_x_min[i] = right_x_min[i + 1].min(xh_sum[i].1);
    }

    for &t_i in &t {
        let boundary_idx = xh_sum.lower_bound(&(t_i, 0, 0));

        let mut ans = 0;
        if boundary_idx > 0 {
            ans = ans.max(left_h_max[boundary_idx - 1]);
        }

        if boundary_idx < n {
            ans = ans.max(t_i - right_x_min[boundary_idx]);
        }

        println!("{}", ans);
    }
}
