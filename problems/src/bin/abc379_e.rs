use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut accumulated_s = vec![0; n + 1];
    for i in 0..n {
        accumulated_s[i + 1] = (s[i] as usize - '0' as usize) * (i + 1) + accumulated_s[i];
    }

    let mut reversed_ans = vec![];
    let mut carried = 0;
    for i in 0..n {
        carried += accumulated_s[n - i];
        reversed_ans.push(std::char::from_digit((carried % 10) as u32, 10).unwrap());
        carried /= 10;
    }

    if carried > 0 {
        let carried_chars = decimal_to_chars(carried, 10);
        reversed_ans.extend(&carried_chars);
    }

    println!("{}", reversed_ans.iter().rev().join(""));
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
