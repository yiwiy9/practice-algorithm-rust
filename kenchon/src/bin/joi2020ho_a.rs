use itertools::Itertools;
use proconio::input;
use superslice::Ext;
const INF: i64 = 1 << 60;

fn main() {
    input! {
        n: usize,
        a: [i64; n+1],
        b: [i64; n],
    }

    let mut a_sorted = a.clone();
    a_sorted.sort();
    let mut b_sorted = b.clone();
    b_sorted.sort();

    let mut left_max = vec![0; n + 1];
    left_max[0] = (a_sorted[0] - b_sorted[0]).max(0);
    for i in 1..n {
        left_max[i] = left_max[i - 1].max(a_sorted[i] - b_sorted[i]);
    }
    left_max[n] = INF;

    let mut right_max = vec![0; n + 1];
    right_max[n] = (a_sorted[n] - b_sorted[n - 1]).max(0);
    for i in (1..n).rev() {
        right_max[i] = right_max[i + 1].max(a_sorted[i] - b_sorted[i - 1]);
    }
    right_max[0] = INF;

    let mut ans = vec![0; n + 1];
    for (i, &a_i) in a.iter().enumerate() {
        let sorted_i = a_sorted.lower_bound(&a_i);
        if sorted_i > 0 {
            ans[i] = ans[i].max(left_max[sorted_i - 1]);
        }
        if sorted_i < n {
            ans[i] = ans[i].max(right_max[sorted_i + 1]);
        }
    }

    println!("{}", ans.iter().join(" "));
}
