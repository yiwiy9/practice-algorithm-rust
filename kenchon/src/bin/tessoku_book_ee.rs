use ac_library::{Min, Segtree};
use proconio::input;
use superslice::*;

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        x: [usize; n],
    }

    let mut min_segtree = Segtree::<Min<usize>>::from_iter(std::iter::repeat_n(INF, n));
    min_segtree.set(0, 0);
    for (i, &x_i) in x.iter().enumerate().skip(1) {
        let left_x = x_i.saturating_sub(r);
        let right_x = x_i.saturating_sub(l);
        let left = x.lower_bound(&left_x);
        let right = x.upper_bound(&right_x);

        if left == right {
            // 空区間を渡すとACLセグ木落ちる
            continue;
        }

        let before_min = min_segtree.prod(left..right);
        min_segtree.set(i, before_min + 1);
    }

    println!("{}", min_segtree.get(n - 1));
}
