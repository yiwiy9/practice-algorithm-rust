use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        q: usize,
        tab: [(usize,usize,usize); q],
    }

    let mut following = BTreeSet::new();
    for (t, a, b) in tab {
        if t == 1 {
            following.insert((a, b));
        } else if t == 2 {
            following.remove(&(a, b));
        } else {
            if following.contains(&(a, b)) && following.contains(&(b, a)) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
