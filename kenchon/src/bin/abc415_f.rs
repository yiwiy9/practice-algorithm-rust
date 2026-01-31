use ac_library::{Monoid, Segtree};
use proconio::{
    input,
    marker::{Chars, Usize1},
};

type CharCnt = ((char, i64), (char, i64), bool, i64);

struct CharCntMonoid;

impl Monoid for CharCntMonoid {
    type S = CharCnt;

    fn identity() -> Self::S {
        (('0', 0), ('0', 0), true, 0)
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        let (a_left, a_right, a_all_eq, a_max) = *a;
        let (b_left, b_right, b_all_eq, b_max) = *b;

        let mut next_left = a_left;
        let mut next_right = b_right;
        let mut next_all_eq = a_all_eq && b_all_eq;
        let mut next_max = a_max.max(b_max);
        if a_right.0 == b_left.0 {
            next_max = next_max.max(a_right.1 + b_left.1);

            if a_all_eq {
                next_left.1 += b_left.1;
                next_max = next_max.max(next_left.1);
            }
            if b_all_eq {
                next_right.1 += a_right.1;
                next_max = next_max.max(next_right.1);
            }
        } else {
            next_all_eq = false;
        }

        (next_left, next_right, next_all_eq, next_max)
    }
}

fn main() {
    input! {
        _n: usize,
        q: usize,
        s: Chars,
    }

    let mut segtree: Segtree<CharCntMonoid> =
        Segtree::from_iter(s.iter().map(|&x| ((x, 1), (x, 1), true, 1)));

    for _ in 0..q {
        input! {
            op: u8,
        }

        match op {
            1 => {
                input! {
                    i: Usize1,
                    x: char,
                }
                segtree.set(i, ((x, 1), (x, 1), true, 1));
            }
            2 => {
                input! {
                    l: Usize1,
                    r: Usize1,
                }
                let (_, _, _, max_cnt) = segtree.prod(l..=r);
                println!("{}", max_cnt);
            }
            _ => unreachable!(),
        }
    }
}
