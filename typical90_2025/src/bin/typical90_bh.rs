use ac_library::{Max, Segtree};
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut front_max_segtree = Segtree::<Max<usize>>::new(n + 1);
    let mut front_max = vec![0; n];
    for i in 0..n {
        front_max[i] = front_max_segtree.prod(0..a[i]);
        front_max_segtree.set(a[i], front_max_segtree.get(a[i]).max(front_max[i] + 1));
    }

    let mut back_max_segtree = Segtree::<Max<usize>>::new(n + 1);
    let mut back_max = vec![0; n];
    for i in (0..n).rev() {
        back_max[i] = back_max_segtree.prod(0..a[i]);
        back_max_segtree.set(a[i], back_max_segtree.get(a[i]).max(back_max[i] + 1));
    }

    let mut ans = 0;
    for i in 0..n {
        ans = ans.max(front_max[i] + back_max[i] + 1);
    }

    println!("{}", ans);
}
