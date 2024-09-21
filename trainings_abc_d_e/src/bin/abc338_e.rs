use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        ab: [(usize,usize); n],
    }

    let mut map = BTreeMap::new();
    for (i, &(a, b)) in ab.iter().enumerate() {
        map.insert(a, i);
        map.insert(b, i);
    }

    let mut no_intersection = true;
    let mut stack = vec![];
    let mut is_pushed = vec![false; n];
    for &i in map.values() {
        if !is_pushed[i] {
            stack.push(i);
            is_pushed[i] = true;
            continue;
        }

        if stack.last() == Some(&i) {
            stack.pop();
        } else {
            no_intersection = false;
            break;
        }
    }

    println!("{}", if no_intersection { "No" } else { "Yes" });
}
