use ac_library::{Additive, Segtree};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
    }

    let mut used_segtree = Segtree::<Additive<usize>>::new(n);
    for i in 0..n {
        used_segtree.set(i, 1);
    }

    let mut ans = vec![0; n];
    for (i, &p_i) in p.iter().enumerate().rev() {
        let pos = used_segtree.max_right(0, |&x| x < p_i + 1); // falseになったタイミングの位置
        ans[pos] = i + 1;
        used_segtree.set(pos, 0);
    }

    println!("{}", ans.iter().join(" "));
}
