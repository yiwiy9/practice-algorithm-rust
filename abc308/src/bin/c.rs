use itertools::Itertools as _;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize,usize); n],
    }

    let mut idx_vec = (0..n).collect::<Vec<_>>();

    let compare = |&i: &usize, &j: &usize| (ab[j].0 * ab[i].1).cmp(&(ab[i].0 * ab[j].1));
    idx_vec.sort_by(compare);

    println!("{}", idx_vec.iter().map(|i| i + 1).join(" "));
}
