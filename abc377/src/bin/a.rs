use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut set = std::collections::HashSet::new();
    set.insert('A');
    set.insert('B');
    set.insert('C');
    for c in s {
        set.remove(&c);
    }

    println!("{}", if set.is_empty() { "Yes" } else { "No" });
}
