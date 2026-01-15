use ac_library::{Additive, Segtree};
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
        s[i + 1] %= m;
    }

    let mut cnt_segtree = Segtree::<Additive<usize>>::new(m);
    for &s_i in s.iter().skip(1) {
        cnt_segtree.set(s_i, cnt_segtree.get(s_i) + 1);
    }

    let mut total_sum = s.iter().sum::<usize>();
    let mut ans: usize = total_sum;
    for &s_i in s.iter().skip(1) {
        total_sum -= s_i;
        cnt_segtree.set(s_i, cnt_segtree.get(s_i) - 1);

        let less_cnt = cnt_segtree.prod(..s_i);
        let remain_cnt = cnt_segtree.all_prod();
        ans += total_sum + m * less_cnt - s_i * remain_cnt;
    }

    println!("{}", ans);
}
