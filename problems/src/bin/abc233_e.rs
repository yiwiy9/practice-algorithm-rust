use itertools::Itertools;
use itertools_num::ItertoolsNum;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
    }

    let mut s = x
        .iter()
        .map(|&c| c as usize - '0' as usize)
        .cumsum()
        .collect::<Vec<usize>>();

    s = s.iter().rev().copied().collect();

    let mut ans = vec![];
    let mut carry = 0;
    for &num in &s {
        let sum = num + carry;
        ans.push(sum % 10);
        carry = sum / 10;
    }

    if carry > 0 {
        ans.push(carry);
    }

    println!("{}", ans.iter().rev().join(""));
}
