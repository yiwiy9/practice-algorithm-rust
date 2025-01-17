use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {
        x: Chars,
        n: usize,
        s: [Chars; n],
    }

    let mut map = HashMap::new();
    for (i, &c) in x.iter().enumerate() {
        map.insert(c, ('a' as u8 + i as u8) as char);
    }

    let mut s_normal = vec![];
    for s_i in s.iter() {
        let mut s_i_normal = vec![];
        for c in s_i {
            s_i_normal.push(map[c]);
        }
        s_normal.push(s_i_normal);
    }

    let mut s_normal_sorted = s_normal
        .iter()
        .enumerate()
        .map(|(i, s_i)| (s_i, i))
        .collect::<Vec<_>>();
    s_normal_sorted.sort();

    for (_, i) in s_normal_sorted {
        println!("{}", s[i].iter().collect::<String>());
    }
}
