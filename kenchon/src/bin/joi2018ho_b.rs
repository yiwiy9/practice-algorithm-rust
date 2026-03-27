use ac_library::{Max, Segtree};
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n],
    }

    ab.sort_by(|a, b| b.0.cmp(&a.0));

    let mut max_segtree = Segtree::<Max<usize>>::new(n);
    let mut b_sum = 0;
    for (i, &(a, b)) in ab.iter().enumerate() {
        b_sum += b;
        max_segtree.set(i, b_sum + a);
    }

    let mut ans: usize = 0;
    let mut b_sub = 0;
    for (i, &(a, b)) in ab.iter().enumerate() {
        let cur_max = max_segtree.prod(i..) - a - b_sub;
        ans = ans.max(cur_max);
        b_sub += b;
    }

    println!("{}", ans);
}
