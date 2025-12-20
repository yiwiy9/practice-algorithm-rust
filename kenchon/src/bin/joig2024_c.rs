use itertools::Itertools;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        cx: [(usize,usize); n],
    }

    let mut min_heap = BinaryHeap::new();
    for (i, &(c, x)) in cx.iter().enumerate() {
        min_heap.push(Reverse((x, c, i)));
    }

    let mut sorted_vec = vec![];
    while let Some(Reverse((x, c, i))) = min_heap.pop() {
        let mut same_colors = vec![(x, i)];
        while let Some(Reverse((_, cc, _))) = min_heap.peek() {
            if *cc != c {
                break;
            }
            let Reverse((xx, _, ii)) = min_heap.pop().unwrap();
            same_colors.push((xx, ii));
        }
        sorted_vec.push(same_colors);
    }

    let m = sorted_vec.len();
    let mut ans = vec![INF; n];
    for i in 0..m {
        for &(cur_x, cur_i) in &sorted_vec[i] {
            if i > 0 {
                ans[cur_i] = ans[cur_i].min(cur_x - sorted_vec[i - 1].last().unwrap().0);
            }
            if i < m - 1 {
                ans[cur_i] = ans[cur_i].min(sorted_vec[i + 1].first().unwrap().0 - cur_x);
            }
        }
    }

    println!("{}", ans.iter().join("\n"));
}
