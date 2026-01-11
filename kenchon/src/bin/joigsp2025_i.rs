use proconio::{input, marker::Usize1};
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; n],
        b: [usize; n],
        c: [Usize1; m],
        d: [usize; m],
    }

    let mut brand_b = vec![BinaryHeap::new(); n];
    for i in 0..n {
        brand_b[a[i]].push(b[i]);
    }

    let mut brand_d = vec![BinaryHeap::new(); n];
    for j in 0..m {
        brand_d[c[j]].push(d[j]);
    }

    // 合計の最大なので、増分が大きいものから貪欲で良い
    // 同じブランド内のものは強いものから組み合わせていく、合計の最大なので組み合わせ方を変えると悪化するのみ
    let mut max_heap = BinaryHeap::new();
    for i in 0..n {
        while let Some(b) = brand_b[i].pop() {
            max_heap.push(b + brand_d[i].pop().unwrap_or(0));
        }
    }

    let mut ans: usize = 0;
    for _ in 0..m {
        ans += max_heap.pop().unwrap();
    }

    println!("{}", ans);
}
