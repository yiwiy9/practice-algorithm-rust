use proconio::input;
use rand_core::le;

fn main() {
    input! {
        n: usize,
        a_s: [(i64, char); n],
    }

    let mut left: Vec<i64> = a_s
        .iter()
        .filter(|(_, c)| *c == 'L')
        .map(|(a, _)| *a)
        .collect();

    let mut right: Vec<i64> = a_s
        .iter()
        .filter(|(_, c)| *c == 'R')
        .map(|(a, _)| *a)
        .collect();

    let mut ans = 0;
    for i in 1..left.len() {
        ans += (left[i] - left[i - 1]).abs();
    }
    for i in 1..right.len() {
        ans += (right[i] - right[i - 1]).abs();
    }

    println!("{}", ans);
}
