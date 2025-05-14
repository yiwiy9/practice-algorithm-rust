use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut set = std::collections::HashSet::new();
    for &c in &s {
        set.insert(c as u8 - b'a');
    }

    for i in 0..26 {
        if !set.contains(&i) {
            println!("{}", (b'a' + i) as char);
            return;
        }
    }
}
