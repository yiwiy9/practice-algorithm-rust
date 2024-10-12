use proconio::{input, marker::Usize1};
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        q: usize,
        tab: [(u8, Usize1, Usize1); q],
    }

    let mut follow = BTreeSet::new();
    for (t, a, b) in tab {
        match t {
            1 => {
                follow.insert((a, b));
            }
            2 => {
                follow.remove(&(a, b));
            }
            3 => {
                if follow.contains(&(a, b)) && follow.contains(&(b, a)) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
            _ => unreachable!(),
        }
    }
}
