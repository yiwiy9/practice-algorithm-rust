use ac_library::{Max, Segtree};
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        a: [usize; n],
    }

    let max_a = a.iter().copied().max().unwrap();

    let mut segtree = Segtree::<Max<usize>>::new(max_a + 1);
    for &a_i in &a {
        let left = if a_i >= d { a_i - d } else { 0 };
        let right = if a_i + d <= max_a { a_i + d } else { max_a };
        let cur_max = segtree.prod(left..=right);
        segtree.set(a_i, cur_max + 1);
    }

    println!("{}", segtree.prod(0..=max_a));
}
