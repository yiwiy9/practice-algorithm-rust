use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut usernames = HashSet::new();
    for (i, s_i) in s.iter().enumerate().take(n) {
        if usernames.contains(s_i) {
            continue;
        }
        println!("{}", i + 1);
        usernames.insert(s_i);
    }
}
