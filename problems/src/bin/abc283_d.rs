use proconio::{input, marker::Chars};
use std::collections::BTreeSet;

fn main() {
    input! {
        s: Chars,
    }

    let mut stack = vec![];
    let mut cur_set = BTreeSet::new();
    for &c in &s {
        match c {
            '(' => {
                stack.push(cur_set.clone());
            }
            ')' => {
                cur_set = stack.pop().unwrap();
            }
            _ => {
                if cur_set.contains(&c) {
                    println!("No");
                    return;
                }
                cur_set.insert(c);
            }
        }
    }

    println!("Yes");
}
