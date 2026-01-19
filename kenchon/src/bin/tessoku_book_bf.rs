use ac_library::{Max, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut segtree = Segtree::<Max<usize>>::new(n);
    for _ in 0..q {
        input! {
            op: usize,
        }

        match op {
            1 => {
                input! {
                    pos: Usize1,
                    x: usize,
                }
                segtree.set(pos, x);
            }
            2 => {
                input! {
                    l: Usize1,
                    r: Usize1,
                }
                println!("{}", segtree.prod(l..r));
            }
            _ => unreachable!(),
        }
    }
}
