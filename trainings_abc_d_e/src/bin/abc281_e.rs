use itertools::Itertools as _;
use proconio::input;
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
    }

    let mut min_heap = BinaryHeap::new();
    for i in 0..m {
        min_heap.push(Reverse((a[i], i)));
    }

    let mut sum = 0;
    let mut max_heap = BinaryHeap::new();
    for _ in 0..k {
        let Reverse((a_i, i)) = min_heap.pop().unwrap();
        sum += a_i;
        max_heap.push((a_i, i));
    }

    let mut consumed = vec![false; n];
    let mut ans = vec![];
    ans.push(sum);
    for i in 0..n - m {
        while let Some(&(a_i, j)) = max_heap.peek() {
            if consumed[j] {
                max_heap.pop();
                continue;
            }
            if a_i > a[i + m] {
                sum -= a_i;
                sum += a[i + m];
                max_heap.pop();
                max_heap.push((a[i + m], i + m));
                min_heap.push(Reverse((a_i, j)));
            } else {
                min_heap.push(Reverse((a[i + m], i + m)));
            }
            break;
        }

        if a[i] <= max_heap.peek().unwrap().0 {
            sum -= a[i];

            while let Some(Reverse((a_i, j))) = min_heap.pop() {
                if consumed[j] {
                    continue;
                }
                sum += a_i;
                max_heap.push((a_i, j));
                break;
            }
        }

        consumed[i] = true;
        ans.push(sum);
    }

    println!("{}", ans.iter().join(" "));
}
