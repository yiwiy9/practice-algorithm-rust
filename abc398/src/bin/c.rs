use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut cnt = HashMap::new();
    for &a_i in &a {
        *cnt.entry(a_i).or_insert(0) += 1;
    }

    let mut unique = vec![];
    for (k, v) in cnt.iter() {
        if *v == 1 {
            unique.push(*k);
        }
    }

    if unique.len() == 0 {
        println!("-1");
        return;
    }

    let max_unique = unique.iter().max().unwrap();
    let mut ans = 0;
    for i in 0..n {
        if a[i] == *max_unique {
            ans = i + 1;
            break;
        }
    }
    println!("{}", ans);
}
