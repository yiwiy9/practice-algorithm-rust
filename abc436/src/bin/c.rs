use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        rc: [(usize,usize); m],
    }

    let mut ans = 0;
    let mut set = BTreeSet::new();
    for &(r, c) in &rc {
        if set.contains(&(r, c)) {
            continue;
        }
        if set.contains(&(r + 1, c)) {
            continue;
        }
        if set.contains(&(r, c + 1)) {
            continue;
        }
        if set.contains(&(r + 1, c + 1)) {
            continue;
        }
        ans += 1;
        set.insert((r, c));
        set.insert((r + 1, c));
        set.insert((r, c + 1));
        set.insert((r + 1, c + 1));
    }

    println!("{}", ans);
}
