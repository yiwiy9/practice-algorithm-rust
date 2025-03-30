use itertools::Itertools;
use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut map = BTreeMap::new();
    for &p_i in &p {
        *map.entry(p_i).or_insert(0) += 1;
    }

    let mut r_vac = vec![0; 101];
    let mut r = 1;
    for (&p, &cnt) in map.iter().rev() {
        r_vac[p] = r;
        r += cnt;
    }

    println!("{}", p.iter().map(|&p_i| r_vac[p_i]).join("\n"));
}
