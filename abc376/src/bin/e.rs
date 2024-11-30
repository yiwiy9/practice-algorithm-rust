use proconio::input;
use std::collections::BinaryHeap;

const INF: usize = 1 << 60;

/// https://atcoder.jp/contests/abc376/tasks/abc376_e
/// https://atcoder.jp/contests/abc376/editorial/11187
/// A が昇順ソートされていると考えると、
/// r = k..nについて、b1..br-1のうちの最小からk-1個選ぶ時の値を全探索すれば良い
/// Ar × ( Br + (B1,B2,…,Br−1のうち小さい方から K−1 個の和) )
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            k: usize,
            a: [usize; n],
            b: [usize; n],
        }

        let mut c = vec![];
        for i in 0..n {
            c.push((a[i], b[i]));
        }
        c.sort();

        let mut ans = INF;
        let mut b_sum = 0;
        let mut b_max_heap = BinaryHeap::new();
        for i in 0..n {
            let (a_i, b_i) = c[i];
            if i + 1 < k {
                b_max_heap.push(b_i);
                b_sum += b_i;
            } else {
                ans = ans.min(a_i * (b_i + b_sum));
                if !b_max_heap.is_empty() && b_max_heap.peek().unwrap() > &b_i {
                    b_sum -= b_max_heap.pop().unwrap();
                    b_max_heap.push(b_i);
                    b_sum += b_i;
                }
            }
        }

        println!("{}", ans);
    }
}
