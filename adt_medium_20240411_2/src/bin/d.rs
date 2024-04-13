use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
    }

    let mut set = BTreeSet::new();
    for i in 1..=1000 {
        for j in 1..=1000 {
            set.insert(4 * i * j + 3 * i + 3 * j);
        }
    }

    let mut ans = 0;
    for s_i in s {
        if !set.contains(&s_i) {
            ans += 1;
        }
    }

    println!("{}", ans);
}
