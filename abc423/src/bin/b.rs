use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        l: [usize; n],
    }

    let mut set = HashSet::new();
    set.insert(0);
    set.insert(n);

    for (i, &li) in l.iter().enumerate() {
        if li == 1 {
            break;
        }
        set.insert(i + 1);
    }
    for (i, &li) in l.iter().enumerate().rev() {
        if li == 1 {
            break;
        }
        set.insert(i);
    }

    println!("{}", n + 1 - set.len());
}
