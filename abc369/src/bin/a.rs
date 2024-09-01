use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        a: i64,
        b: i64,
    }

    let mut set = HashSet::new();

    set.insert(b + (b - a));
    set.insert(a + (a - b));

    if (a + b) % 2 == 0 {
        set.insert((a + b) / 2);
    }

    set.insert(a - (b - a));
    set.insert(b - (a - b));

    println!("{}", set.len());
}
