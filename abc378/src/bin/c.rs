use itertools::Itertools as _;
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();
    let mut b = vec![-1; n];
    for (i, &a_i) in a.iter().enumerate() {
        if let Some(j) = map.get(&a_i) {
            b[i] = *j as i64 + 1;
        }
        map.insert(a_i, i);
    }

    println!("{}", b.iter().join(" "));
}
