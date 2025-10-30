use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut map = BTreeMap::new();
    for &a_i in &a {
        *map.entry(a_i).or_insert(0) += 1;
    }

    let mut target_num = -1;
    for (&num, &cnt) in map.iter().rev() {
        if cnt == 1 {
            target_num = num;
            break;
        }
    }

    for (i, &a_i) in a.iter().enumerate() {
        if a_i == target_num {
            println!("{}", i + 1);
            return;
        }
    }

    println!("-1");
}
