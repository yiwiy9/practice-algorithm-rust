use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: Usize1,
        q: Usize1,
        r: Usize1,
        s: Usize1,
        a: [usize; n],
    }

    let b = a[0..p]
        .iter()
        .chain(a[r..=s].iter())
        .chain(a[q + 1..r].iter())
        .chain(a[p..=q].iter())
        .chain(a[s + 1..n].iter())
        .copied()
        .collect_vec();

    println!("{}", b.iter().join(" "));
}
