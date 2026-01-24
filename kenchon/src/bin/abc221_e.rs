use ac_library::{FenwickTree, ModInt998244353 as Mint};
use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut a_sorted = a.clone();
    a_sorted.sort();
    a_sorted.dedup();

    let mut mod_sum_ft = FenwickTree::new(a_sorted.len(), Mint::new(0));
    let mut ans = Mint::new(0);
    for (i, a_i) in a.iter().enumerate() {
        let a_i_order = a_sorted.lower_bound(a_i);
        let mod_sum = mod_sum_ft.sum(0..=a_i_order);

        let cur_sum = Mint::new(2).pow((n - i) as u64);
        ans += mod_sum / cur_sum;

        mod_sum_ft.add(a_i_order, cur_sum / 2);
    }

    println!("{}", ans);
}
