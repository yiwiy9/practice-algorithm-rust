use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut map = HashMap::new();
    for (i, &a_i) in a.iter().enumerate() {
        map.insert(a_i, i + 1);
    }

    let mut ans = vec![];
    let mut i = -1;
    while let Some(&j) = map.get(&i) {
        ans.push(j);
        i = j as i32;
    }

    println!("{}", ans.iter().join(" "));
}
