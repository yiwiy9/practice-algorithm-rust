use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    if s.len() % 2 == 1 {
        println!("No");
        return;
    }

    for i in 1..=s.len() / 2 {
        if s[2 * i - 2] != s[2 * i - 1] {
            println!("No");
            return;
        }
    }

    let mut map = std::collections::HashMap::new();
    for c in s {
        *map.entry(c).or_insert(0) += 1;
    }

    for (_, &v) in map.iter() {
        if v != 2 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
