use ac_library::FenwickTree;
use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut b = a.clone();
    b.sort();

    let mut ft_sum = FenwickTree::new(n, 0);
    let mut ft_cnt = FenwickTree::new(n, 0);
    let mut ans: usize = 0;
    for &a_i in &a {
        let b_i = b.lower_bound(&a_i);

        let lower_sum = ft_sum.sum(0..b_i);
        let lower_cnt = ft_cnt.sum(0..b_i);
        ans += a_i * lower_cnt - lower_sum;

        ft_sum.add(b_i, a_i);
        ft_cnt.add(b_i, 1);
    }

    println!("{}", ans);
}
