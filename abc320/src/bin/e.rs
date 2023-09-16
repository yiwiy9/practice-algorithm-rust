use itertools::Itertools;
use proconio::input;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};

/**
 * https://atcoder.jp/contests/abc320/tasks/abc320_e
 * https://atcoder.jp/contests/abc320/editorial/7125
 * abc221_d に引っ張られた（カウンター増減, imos法っぽく）
 * => 今回はimos法っぽいことは不要
 * => 線ではなく、点
 * => 素直にpriority_queueで時刻ごとに点を管理
 */
fn main() {
    input! {
        n: usize,
        m: usize,
        tws: [(usize,usize,usize); m],
    }

    let mut heap = BinaryHeap::new();
    for (i, &(t, _, _)) in tws.iter().enumerate() {
        heap.push(Reverse((t, 1, i)));
    }

    let mut ans = vec![0; n];
    let mut set: BTreeSet<_> = (0..n).collect();
    while let Some(Reverse((t, op, i))) = heap.pop() {
        match op {
            0 => {
                set.insert(i);
                continue;
            }
            1 => {
                if let Some(eater) = set.pop_first() {
                    ans[eater] += tws[i].1;
                    heap.push(Reverse((t + tws[i].2, 0, eater)));
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ans.iter().join("\n"));
}
