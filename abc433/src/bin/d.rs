use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let mut map = BTreeMap::new();
    for &a_i in &a {
        let a_i_chars = decimal_to_chars(a_i, 10);
        map.entry((a_i_chars.len(), a_i % m))
            .and_modify(|cur| *cur += 1)
            .or_insert(1);
    }

    let mut ans = 0_usize;
    for &a_i in &a {
        for d in 1..=10 {
            let mut a_i_mod = a_i;
            a_i_mod %= m;
            for _ in 0..d {
                a_i_mod *= 10;
                a_i_mod %= m;
            }
            let target = (m - a_i_mod) % m;
            if let Some(&cnt) = map.get(&(d as usize, target)) {
                ans += cnt;
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
