use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        l: usize,
        q: usize,
        cx: [(usize,usize); q],
    }

    let mut set = BTreeSet::new();
    set.insert(0);
    set.insert(l);
    for &(c, x) in &cx {
        match c {
            1 => {
                set.insert(x);
            }
            2 => {
                let front = set.range(0..x).last().unwrap();
                let end = set.range(x..).next().unwrap();
                println!("{}", end - front);
            }
            _ => unreachable!(),
        }
    }
}
