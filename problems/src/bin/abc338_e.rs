use proconio::{input, marker::Usize1};
use std::collections::BTreeMap;

/// https://atcoder.jp/contests/abc338/tasks/abc338_e
/// https://atcoder.jp/contests/abc338/editorial/9172
/// 円環の弦同士の交点を考える問題においては、切り開いて一直線上に考えるというアイデアが頻出
/// => この発想さえ出れば、あとはスタックで管理すれば交わってるかすぐ判定可能
fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n],
    }

    let mut event_map = BTreeMap::new();
    for (i, &(a, b)) in ab.iter().enumerate() {
        event_map.insert(a, (i, if a < b { 1 } else { -1 }));
        event_map.insert(b, (i, if a >= b { 1 } else { -1 }));
    }

    let mut stack = vec![];
    for &(idx, flg) in event_map.values() {
        if flg == 1 {
            stack.push(idx);
        } else {
            let v = stack.pop().unwrap();
            if v != idx {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
