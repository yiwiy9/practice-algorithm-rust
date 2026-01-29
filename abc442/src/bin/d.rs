use ac_library::{Additive, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let mut sum_segtree = Segtree::<Additive<usize>>::from(a);
    for _ in 0..q {
        input! {
            op:usize,
        }

        if op == 1 {
            input! {
                x:Usize1,
            }
            let x_val = sum_segtree.get(x);
            let x_plus_val = sum_segtree.get(x + 1);
            sum_segtree.set(x, x_plus_val);
            sum_segtree.set(x + 1, x_val);
        } else {
            input! {
                l:Usize1,
                r:Usize1,
            }
            println!("{}", sum_segtree.prod(l..=r));
        }
    }
}
