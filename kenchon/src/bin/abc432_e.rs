use ac_library::{Additive, Segtree};
use proconio::{input, marker::Usize1};

const MAX_VAL: usize = 500_010;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
    }

    let mut sum_segtree = Segtree::<Additive<usize>>::new(MAX_VAL);
    let mut cnt_segtree = Segtree::<Additive<usize>>::new(MAX_VAL);
    for &a_i in &a {
        sum_segtree.set(a_i, sum_segtree.get(a_i) + a_i);
        cnt_segtree.set(a_i, cnt_segtree.get(a_i) + 1);
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
                let a_i = a[x];
                sum_segtree.set(a_i, sum_segtree.get(a_i) - a_i);
                cnt_segtree.set(a_i, cnt_segtree.get(a_i) - 1);

                a[x] = y;
                let a_i = a[x];
                sum_segtree.set(a_i, sum_segtree.get(a_i) + a_i);
                cnt_segtree.set(a_i, cnt_segtree.get(a_i) + 1);
            }
            2 => {
                input! {
                    l: usize,
                    r: usize,
                }

                let mut ans = 0;
                if l < r {
                    ans += sum_segtree.prod(l..=r);
                    ans += r * cnt_segtree.prod(r + 1..);
                    ans += l * cnt_segtree.prod(..l);
                } else {
                    ans = l * n;
                }
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}
