use proconio::input;
use std::{cmp::Reverse, collections::BinaryHeap};

/**
 * https://atcoder.jp/contests/abc127/tasks/abc127_f
 * 昇順に並べた全ての変化点の内、
 * ちょうど真ん中にある変化点（N が偶数の場合には真ん中にある 2 つの変化点の間）において
 * f(x) は傾きが 0 となり、最小値を取ります。
 * => 左側と右側に分けて、それぞれ set で管理する
 */
fn main() {
    input! {
        q: usize,
    }

    let mut b_sum = 0;
    let mut left_heap = BinaryHeap::new();
    let mut left_heap_sum = 0;
    let mut right_heap = BinaryHeap::new();
    let mut right_heap_sum = 0;

    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! {
                    a: i64,
                    b: i64,
                }
                b_sum += b;

                if left_heap.is_empty() || a > *left_heap.peek().unwrap() {
                    right_heap.push(Reverse(a));
                    right_heap_sum += a;
                } else {
                    left_heap.push(a);
                    left_heap_sum += a;
                }

                if left_heap.len() > right_heap.len() + 1 {
                    let v = left_heap.pop().unwrap();
                    left_heap_sum -= v;
                    right_heap.push(Reverse(v));
                    right_heap_sum += v;
                }
                if right_heap.len() > left_heap.len() {
                    let Reverse(v) = right_heap.pop().unwrap();
                    right_heap_sum -= v;
                    left_heap.push(v);
                    left_heap_sum += v;
                }
            }
            2 => {
                let median = *left_heap.peek().unwrap();
                let total = -left_heap_sum + median * left_heap.len() as i64 + right_heap_sum
                    - median * right_heap.len() as i64
                    + b_sum;
                println!("{} {}", median, total);
            }
            _ => unreachable!(),
        }
    }
}
