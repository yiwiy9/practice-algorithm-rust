use ac_library::{Additive, Segtree};
use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        txy: [(usize, Usize1, i64); q],
    }

    let mut a_sorted = vec![0];
    let mut b_sorted = vec![0];
    for &(t, _, y) in &txy {
        if t == 1 {
            a_sorted.push(y);
        } else {
            b_sorted.push(y);
        }
    }
    a_sorted.sort();
    a_sorted.dedup();
    b_sorted.sort();
    b_sorted.dedup();

    let mut a_cnt_segtree = Segtree::<Additive<i64>>::new(a_sorted.len());
    let mut a_sum_segtree = Segtree::<Additive<i64>>::new(a_sorted.len());
    let mut b_cnt_segtree = Segtree::<Additive<i64>>::new(b_sorted.len());
    let mut b_sum_segtree = Segtree::<Additive<i64>>::new(b_sorted.len());

    a_cnt_segtree.set(0, n as i64);
    b_cnt_segtree.set(0, m as i64);
    let mut a: Vec<i64> = vec![0; n];
    let mut b: Vec<i64> = vec![0; m];

    let mut ans: i64 = 0;
    for &(t, x, y) in &txy {
        if t == 1 {
            let before_num = a[x];
            let before_num_i = b_sorted.lower_bound(&before_num);
            let before_sum = before_num * b_cnt_segtree.prod(..before_num_i)
                + b_sum_segtree.prod(before_num_i..);
            ans -= before_sum;

            let after_num = y;
            let after_num_i = b_sorted.lower_bound(&after_num);
            let after_sum =
                after_num * b_cnt_segtree.prod(..after_num_i) + b_sum_segtree.prod(after_num_i..);
            ans += after_sum;

            let before_same_num_i = a_sorted.lower_bound(&before_num);
            a_cnt_segtree.set(before_same_num_i, a_cnt_segtree.get(before_same_num_i) - 1);
            a_sum_segtree.set(
                before_same_num_i,
                a_sum_segtree.get(before_same_num_i) - before_num,
            );

            let after_same_num_i = a_sorted.lower_bound(&after_num);
            a_cnt_segtree.set(after_same_num_i, a_cnt_segtree.get(after_same_num_i) + 1);
            a_sum_segtree.set(
                after_same_num_i,
                a_sum_segtree.get(after_same_num_i) + after_num,
            );

            a[x] = y;
        } else {
            let before_num = b[x];
            let before_num_i = a_sorted.lower_bound(&before_num);
            let before_sum = before_num * a_cnt_segtree.prod(..before_num_i)
                + a_sum_segtree.prod(before_num_i..);
            ans -= before_sum;

            let after_num = y;
            let after_num_i = a_sorted.lower_bound(&after_num);
            let after_sum =
                after_num * a_cnt_segtree.prod(..after_num_i) + a_sum_segtree.prod(after_num_i..);
            ans += after_sum;

            let before_same_num_i = b_sorted.lower_bound(&before_num);
            b_cnt_segtree.set(before_same_num_i, b_cnt_segtree.get(before_same_num_i) - 1);
            b_sum_segtree.set(
                before_same_num_i,
                b_sum_segtree.get(before_same_num_i) - before_num,
            );

            let after_same_num_i = b_sorted.lower_bound(&after_num);
            b_cnt_segtree.set(after_same_num_i, b_cnt_segtree.get(after_same_num_i) + 1);
            b_sum_segtree.set(
                after_same_num_i,
                b_sum_segtree.get(after_same_num_i) + after_num,
            );

            b[x] = y;
        }
        println!("{}", ans);
    }
}
