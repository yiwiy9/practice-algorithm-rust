use ac_library::{Max, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        av: [(Usize1, i64); n],
    }

    let mut max_segtree = Segtree::<Max<i64>>::from(vec![0; m]);
    for &(a, v) in &av {
        let left_max = if a > 0 { max_segtree.prod(..a) } else { 0 };
        let right_max = if a < m - 1 {
            max_segtree.prod(a + 1..)
        } else {
            0
        };

        let next_v = left_max.max(right_max) + v;

        if max_segtree.get(a) < next_v {
            max_segtree.set(a, next_v)
        }
    }

    println!("{}", max_segtree.all_prod());
}
