use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut min_heap = BinaryHeap::new();
    for (i, s_i) in s.iter().enumerate() {
        let cnt = s_i.iter().filter(|&&c| c == 'o').count();
        min_heap.push((cnt, Reverse(i + 1)));
    }

    let mut ans = vec![];
    while let Some((_, Reverse(i))) = min_heap.pop() {
        ans.push(i);
    }

    println!("{}", ans.iter().join(" "));
}
