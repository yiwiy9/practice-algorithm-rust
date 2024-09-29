use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {
        s: Chars,
    }

    let mut map = HashMap::new();
    for (i, &c) in s.iter().enumerate() {
        map.insert(c, i as i64);
    }

    let mut ans = 0;
    let mut cur = map[&'A'];
    for i in 1..26 {
        let c = (b'A' + i) as char;
        let next = *map.get(&c).unwrap();

        ans += (next - cur).abs();
        cur = next;
    }

    println!("{}", ans);
}
