use ac_library::{Monoid, Segtree};
use proconio::{input, marker::Usize1};
use std::cmp::{max, min};

struct SecondLargest;
impl Monoid for SecondLargest {
    type S = (i64, usize, i64, usize);
    fn identity() -> Self::S {
        (-1, 0, -2, 0)
    }
    fn binary_operation(l: &Self::S, r: &Self::S) -> Self::S {
        let first_num = max(l.0, r.0);
        let second_num = max(if l.0 == r.0 { 0 } else { min(l.0, r.0) }, max(l.2, r.2));
        let first_cnt = if l.0 == first_num { l.1 } else { 0 }
            + if r.0 == first_num { r.1 } else { 0 }
            + if l.2 == first_num { l.3 } else { 0 }
            + if r.2 == first_num { r.3 } else { 0 };
        let second_cnt = if l.0 == second_num { l.1 } else { 0 }
            + if r.0 == second_num { r.1 } else { 0 }
            + if l.2 == second_num { l.3 } else { 0 }
            + if r.2 == second_num { r.3 } else { 0 };
        (first_num, first_cnt, second_num, second_cnt)
    }
}

/// https://atcoder.jp/contests/abc343/tasks/abc343_f
/// この問題は，セグメント木に複数の値の組を載せることによって解きます．
///
/// セグ木は左側区間と右側区間の値をマージすることで，親ノードの値を計算する
/// => これが期待する形になるように、binary_operation を実装する
/// 考え方の詳細は以下のリンクを参照↓
/// https://atcoder.jp/contests/abc343/editorial/9440
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
    }

    let mut segtree = Segtree::<SecondLargest>::new(n);
    for (i, &a_i) in a.iter().enumerate() {
        segtree.set(i, (a_i, 1, -1, 0));
    }

    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    p: Usize1,
                    x: i64,
                }
                segtree.set(p, (x, 1, -1, 0));
            }
            2 => {
                input! {
                    l: Usize1,
                    r: Usize1,
                }
                let (_, _, _, second_cnt) = segtree.prod(l..=r);
                println!("{}", second_cnt);
            }
            _ => unreachable!(),
        }
    }
}
