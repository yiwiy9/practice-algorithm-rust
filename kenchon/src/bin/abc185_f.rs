use ac_library::{Monoid, Segtree};
use proconio::{input, marker::Usize1};

struct Xor;
impl Monoid for Xor {
    type S = usize;
    fn identity() -> Self::S {
        0
    }
    fn binary_operation(l: &Self::S, r: &Self::S) -> Self::S {
        l ^ r
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let mut xor_segtree = Segtree::<Xor>::from(a);
    for _ in 0..q {
        input! {
            op: usize,
        }

        match op {
            1 => {
                input! {
                    x: Usize1,
                    y: usize,
                }
                xor_segtree.set(x, xor_segtree.get(x) ^ y);
            }
            2 => {
                input! {
                    x: Usize1,
                    y: Usize1,
                }
                println!("{}", xor_segtree.prod(x..=y));
            }
            _ => unreachable!(),
        }
    }
}
