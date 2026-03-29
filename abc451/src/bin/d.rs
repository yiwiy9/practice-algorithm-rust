use itertools::Itertools;
use proconio::input;
use std::collections::BTreeSet;

const MAX: usize = 1_000_000_000;

/// https://atcoder.jp/contests/abc451/tasks/abc451_d
/// https://atcoder.jp/contests/abc451/editorial/18051
/// 数字（文字）を任意の順番で繋げる全探索
/// => DFS
fn main() {
    input! {
        n: usize,
    }

    let mut powers = vec![];
    for i in 0..30 {
        powers.push(1 << i);
    }

    let mut res = BTreeSet::new();

    for &power in &powers {
        dfs(&powers, &mut res, power);
    }

    let ans = res.iter().collect_vec();

    println!("{}", ans[n - 1]);
}

fn dfs(powers: &Vec<usize>, res: &mut BTreeSet<usize>, cur: usize) {
    if res.contains(&cur) {
        return;
    }
    res.insert(cur);

    for &power in powers {
        let next = concat_in_base(cur, power, 10);
        if next <= MAX {
            dfs(powers, res, next);
        }
    }
}

pub fn pow_base(base: usize, exp: usize) -> usize {
    assert!(base >= 2);
    let mut result = 1usize;
    for _ in 0..exp {
        result *= base;
    }
    result
}
pub fn num_digits(mut n: usize, base: usize) -> usize {
    assert!(base >= 2);
    if n == 0 {
        return 1;
    }
    let mut digits = 0;
    while n > 0 {
        digits += 1;
        n /= base;
    }
    digits
}
pub fn concat_in_base(a: usize, b: usize, base: usize) -> usize {
    a * pow_base(base, num_digits(b, base)) + b
}
