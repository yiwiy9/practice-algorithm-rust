use itertools::Itertools;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    }

    let mut min_heap = BinaryHeap::new();
    for (i, &a_i) in a.iter().enumerate() {
        let i = i as i64 + 1;
        if a_i < 0 {
            let div = ((-a_i + i - 1) / i) as usize;
            if div <= m {
                min_heap.push(Reverse((a_i + i * div as i64, div - 1, i)));
            }
        } else {
            min_heap.push(Reverse((a_i + i, 0, i)));
        }
    }

    let mut mex_vec = vec![0; m];
    while let Some(Reverse((a_i, div, i))) = min_heap.pop() {
        if a_i == mex_vec[div] {
            mex_vec[div] += 1;
        }

        // a_i + i < n の条件で計算量が調和級数になる
        // i が 1, 2, 3, ... と増えるごとに 生き残る要素数が 1, 1/2, 1/3, ... と減少する
        if div + 1 < m && a_i + i < n as i64 {
            min_heap.push(Reverse((a_i + i, div + 1, i)));
        }
    }

    println!("{}", mex_vec.iter().join("\n"));
}
