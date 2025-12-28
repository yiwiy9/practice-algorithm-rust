use proconio::input;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        n: usize,
        _a: usize,
        _b: usize,
        origamis: [(usize,usize,usize,usize); n],
    }

    let mut map = HashMap::new();
    for &(p, q, r, s) in &origamis {
        for i in p..=r {
            for j in q..=s {
                *map.entry((i, j)).or_insert(0_usize) += 1;
            }
        }
    }

    let mut heap = BinaryHeap::new();
    for &cnt in map.values() {
        heap.push(cnt);
    }

    let ans_cnt = heap.pop().unwrap();
    let mut ans_sum = 1;
    while let Some(cnt) = heap.pop() {
        if ans_cnt == cnt {
            ans_sum += 1;
        } else {
            break;
        }
    }

    println!("{}", ans_cnt);
    println!("{}", ans_sum);
}
