use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut ans = 0;
    let mut set = BTreeSet::new();
    for s_i in s {
        if !set.contains(&s_i) {
            ans += 1;
        }
        set.insert(s_i.clone());
        set.insert(s_i.clone().chars().rev().collect::<String>());
    }

    println!("{}", ans);
}
