use ac_library::{Additive, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
    }

    let mut cnt_segtree = Segtree::<Additive<usize>>::new(500_010);
    let mut num_segtree = Segtree::<Additive<usize>>::new(500_010);
    for &a_i in &a {
        cnt_segtree.set(a_i, cnt_segtree.get(a_i) + 1);
        num_segtree.set(a_i, num_segtree.get(a_i) + a_i);
    }

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
                cnt_segtree.set(a[x], cnt_segtree.get(a[x]) - 1);
                num_segtree.set(a[x], num_segtree.get(a[x]) - a[x]);

                cnt_segtree.set(y, cnt_segtree.get(y) + 1);
                num_segtree.set(y, num_segtree.get(y) + y);
                a[x] = y;
            }
            2 => {
                input! {
                    l: usize,
                    r: usize,
                }

                if l > r {
                    println!("{}", n * l);
                } else {
                    let r_cnt = cnt_segtree.prod(r..);
                    let l_cnt = cnt_segtree.prod(..l);
                    let sum = num_segtree.prod(l..r);
                    println!("{}", r_cnt * r + l_cnt * l + sum);
                }
            }
            _ => unreachable!(),
        }
    }
}
