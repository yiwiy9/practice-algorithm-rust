use ac_library::{Additive, Segtree};
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + a[i];
        acc[i + 1] %= m;
    }

    let mut cnt_segtree: Segtree<Additive<usize>> = Segtree::<Additive<usize>>::new(m + 1);
    let mut num_segtree: Segtree<Additive<usize>> = Segtree::<Additive<usize>>::new(m + 1);

    for &acc_i in &acc {
        cnt_segtree.set(acc_i, cnt_segtree.get(acc_i) + 1);
        num_segtree.set(acc_i, num_segtree.get(acc_i) + acc_i);
    }

    let mut ans = 0;
    for &acc_i in &acc {
        cnt_segtree.set(acc_i, cnt_segtree.get(acc_i) - 1);
        num_segtree.set(acc_i, num_segtree.get(acc_i) - acc_i);

        let less_cnt = cnt_segtree.prod(0..acc_i);
        let greater_cnt = cnt_segtree.prod(acc_i + 1..m);
        let less_sum = num_segtree.prod(0..acc_i);
        let greater_sum = num_segtree.prod(acc_i + 1..m);

        ans += greater_sum - greater_cnt * acc_i;
        ans += less_sum + m * less_cnt - less_cnt * acc_i;
    }

    println!("{}", ans);
}
