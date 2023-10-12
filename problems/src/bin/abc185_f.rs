use ac_library::{Monoid, Segtree};
use proconio::input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BitXor(usize);

impl Monoid for BitXor {
    type S = usize;

    // ビットXORの単位元は0
    fn identity() -> Self::S {
        0
    }

    // ビットXORの組み合わせは単純にXOR演算
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        a ^ b
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        txy: [(usize,usize,usize); q],
    }

    let mut segtree = Segtree::<BitXor>::new(n);
    for (i, &a_i) in a.iter().enumerate() {
        segtree.set(i, a_i);
    }

    for (t, x, y) in txy {
        match t {
            1 => segtree.set(x - 1, segtree.get(x - 1) ^ y),
            2 => println!("{}", segtree.prod(x - 1..=y - 1)),
            _ => unreachable!(),
        }
    }
}
