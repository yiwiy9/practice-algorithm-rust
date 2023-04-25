use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: u32,
        x: i64,
        a: [i64;n],
    }

    let mut set = BTreeSet::new();
    for &a_i in &a {
        set.insert(a_i);
    }

    let mut ok = false;
    for &a_i in &a {
        let b = x + a_i;
        if set.contains(&b) {
            ok = true;
            break;
        }
    }

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
