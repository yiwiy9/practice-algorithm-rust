use ac_library::{Max, Segtree};
use proconio::input;

const MAX: usize = 500_010;

fn main() {
    input! {
        n: usize,
        d: usize,
        a: [usize; n],
    }

    let mut max_segtree = Segtree::<Max<usize>>::new(MAX + 1);
    for &a_i in &a {
        let l = a_i.saturating_sub(d);
        let r = (a_i + d).min(MAX);
        max_segtree.set(a_i, max_segtree.prod(l..=r) + 1);
    }

    println!("{}", max_segtree.all_prod());
}
