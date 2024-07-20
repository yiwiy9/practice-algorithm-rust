use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        d: [usize; n],
    }

    let c = a + b;
    let d_mod = d
        .iter()
        .map(|&x| x % c)
        .unique()
        .sorted()
        .collect::<Vec<_>>();

    let mut max_gap = 0;
    for i in 0..d_mod.len() {
        let gap = if i == d_mod.len() - 1 {
            d_mod[0] + c - d_mod[i]
        } else {
            d_mod[i + 1] - d_mod[i]
        };
        max_gap = max_gap.max(gap);
    }

    println!("{}", if max_gap > b { "Yes" } else { "No" });
}
