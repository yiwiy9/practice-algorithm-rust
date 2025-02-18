use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1,Usize1); m],
    }

    let mut set = HashSet::new();
    let mut ans = 0;
    for (a, b) in ab {
        if a == b || set.contains(&(a, b)) || set.contains(&(b, a)) {
            ans += 1;
        } else {
            set.insert((a, b));
        }
    }

    println!("{}", ans);
}
