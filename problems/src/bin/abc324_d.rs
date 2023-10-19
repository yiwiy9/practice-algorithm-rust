use num_integer::Roots;
use proconio::{input, marker::Chars};
use std::collections::BTreeMap;

fn main() {
    input! {
        _: usize,
        mut s: Chars,
    }

    let mut char_cnt = BTreeMap::new();
    for c in s {
        char_cnt.entry(c).and_modify(|cur| *cur += 1).or_insert(1);
    }

    let mut ans = 0;
    for i in 0..(10_000_000_000_000).sqrt() {
        let square = decimal_to_chars(i * i, 10);

        let mut ok = true;
        let mut cur_char_cnt = char_cnt.clone();
        for c in &square {
            if !cur_char_cnt.contains_key(c) {
                ok = false;
                break;
            }

            let &cnt = cur_char_cnt.get(c).unwrap();
            if cnt == 1 {
                cur_char_cnt.remove(c);
            } else {
                cur_char_cnt.insert(*c, cnt - 1);
            }
        }

        cur_char_cnt.remove(&'0');
        if ok && cur_char_cnt.is_empty() {
            ans += 1;
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
