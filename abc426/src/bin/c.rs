use ac_library::{Additive, Segtree};
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        xy: [(usize,usize); q],
    }

    let mut cnt_segtree = Segtree::<Additive<usize>>::new(n + 1);
    for i in 1..=n {
        cnt_segtree.set(i, 1);
    }

    let mut min_version = 0;
    for &(x, y) in &xy {
        if min_version >= x {
            println!("0");
            continue;
        }
        let cnt = cnt_segtree.prod(min_version + 1..=x);
        cnt_segtree.set(y, cnt_segtree.get(y) + cnt);
        min_version = x;
        println!("{}", cnt);
    }
}
