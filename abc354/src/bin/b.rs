use itertools::Itertools as _;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut sc: [(Chars,usize); n],
    }

    sc.sort();
    let t = sc.iter().map(|(_, c)| *c).sum::<usize>();

    println!("{}", sc[t % n].0.iter().join(""));
}
