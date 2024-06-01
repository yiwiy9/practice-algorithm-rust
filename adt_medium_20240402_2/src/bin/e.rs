use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        xk: [(usize,usize); q],
    }

    let mut num_counts = BTreeMap::new();
    for (i, &a_i) in a.iter().enumerate() {
        num_counts.entry(a_i).or_insert(vec![]).push(i + 1);
    }

    for &(x, k) in &xk {
        if !num_counts.contains_key(&x) || num_counts.get(&x).unwrap().len() < k {
            println!("-1");
            continue;
        }
        println!("{}", num_counts.get(&x).unwrap()[k - 1])
    }
}
