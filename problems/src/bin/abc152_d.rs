use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
    }

    let mut map = BTreeMap::new();
    for i in 1..=n {
        let i_chars = decimal_to_chars(i, 10);
        map.entry((
            i_chars[0] as usize - '0' as usize,
            i_chars[i_chars.len() - 1] as usize - '0' as usize,
        ))
        .and_modify(|cur| *cur += 1)
        .or_insert(1);
    }

    let mut ans = 0;

    for j in 1..=9 {
        for k in 1..=9 {
            if map.contains_key(&(j, k)) && map.contains_key(&(k, j)) {
                ans += map.get(&(j, k)).unwrap() * map.get(&(k, j)).unwrap();
            }
        }
    }

    println!("{}", ans);
}

pub fn decimal_to_chars(mut n: usize, base: usize) -> Vec<char> {
    if n == 0 {
        return vec!['0'];
    }
    let mut result = Vec::new();
    while n > 0 {
        let char = std::char::from_digit((n % base) as u32, base as u32).unwrap();
        result.push(char);
        n /= base;
    }
    result.iter().rev().copied().collect()
}
