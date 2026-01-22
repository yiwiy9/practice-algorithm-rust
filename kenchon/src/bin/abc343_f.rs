use ac_library::{Monoid, Segtree};
use proconio::{input, marker::Usize1};

struct LargestPair;
impl Monoid for LargestPair {
    type S = ((i64, i64), (i64, i64));
    fn identity() -> Self::S {
        ((0, 0), (0, 0))
    }
    fn binary_operation(l: &Self::S, r: &Self::S) -> Self::S {
        let (l_first, l_second) = *l;
        let (r_first, r_second) = *r;

        fn add(first: &mut (i64, i64), second: &mut (i64, i64), v: (i64, i64)) {
            if first.0 == v.0 {
                first.1 += v.1;
                return;
            }
            if second.0 == v.0 {
                second.1 += v.1;
                return;
            }

            if v.0 > first.0 {
                *second = *first;
                *first = v;
            } else if v.0 > second.0 {
                *second = v;
            }
        }

        let mut first = (i64::MIN, 0);
        let mut second = (i64::MIN, 0);

        add(&mut first, &mut second, l_first);
        add(&mut first, &mut second, l_second);
        add(&mut first, &mut second, r_first);
        add(&mut first, &mut second, r_second);

        (first, second)
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
    }

    let mut largest_pair_segtree = Segtree::<LargestPair>::new(n);
    for (i, &a_i) in a.iter().enumerate() {
        largest_pair_segtree.set(i, ((a_i, 1), (0, 0)));
    }

    for _ in 0..q {
        input! {
            op: usize,
        }
        match op {
            1 => {
                input! {
                    p: Usize1,
                    x: i64,
                }
                largest_pair_segtree.set(p, ((x, 1), (0, 0)));
            }
            2 => {
                input! {
                    l: Usize1,
                    r: Usize1,
                }
                println!("{}", largest_pair_segtree.prod(l..=r).1 .1);
            }
            _ => unreachable!(),
        }
    }
}
