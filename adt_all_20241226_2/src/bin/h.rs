use std::cmp::Reverse;

use proconio::input;

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

        let mut b_min_heap = std::collections::BinaryHeap::new();
        for (i, &b_i) in b.iter().enumerate() {
            b_min_heap.push(Reverse((b_i, i)));
        }

        let mut b_sum = 0;
        let mut a_max_heap = std::collections::BinaryHeap::new();
        for _ in 0..k {
            let Reverse((b_i, i)) = b_min_heap.pop().unwrap();
            b_sum += b_i;
            a_max_heap.push((a[i], i));
        }
        let mut a_max = a_max_heap.peek().unwrap().0;

        let mut ans = a_max * b_sum;
        while let Some(Reverse((b_i, i))) = b_min_heap.pop() {
            let (_, j) = a_max_heap.pop().unwrap();
            b_sum += b_i - b[j];
            a_max_heap.push((a[i], i));
            a_max = a_max_heap.peek().unwrap().0;
            ans = ans.min(a_max * b_sum);
        }

        println!("{}", ans);
    }
}
