use ac_library::{Max, Segtree};
use proconio::input;

const MAX_SIZE: usize = 500_001;

fn main() {
    input! {
        n: usize,
        d: usize,
        a: [usize; n],
    }

    let mut segtree = Segtree::<Max<usize>>::new(MAX_SIZE);
    for &a_i in &a {
        let left = if a_i >= d { a_i - d } else { 0 };
        let right = if a_i + d < MAX_SIZE {
            a_i + d
        } else {
            MAX_SIZE - 1
        };

        let cur_max = segtree.prod(left..=right);
        segtree.set(a_i, cur_max + 1);
    }

    println!("{}", segtree.all_prod());
}
