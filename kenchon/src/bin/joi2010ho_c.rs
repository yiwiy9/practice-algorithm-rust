use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        l: usize,
        a: [usize; n],
    }

    let is_target = |i: usize, fallen: &mut Vec<bool>| -> bool {
        let mut is_target = true;

        if i > 0 && !fallen[i - 1] && a[i - 1] > a[i] {
            is_target = false;
        }
        if i < n - 1 && !fallen[i + 1] && a[i + 1] > a[i] {
            is_target = false;
        }

        is_target
    };

    let mut fallen = vec![false; n];
    let mut min_heap = BinaryHeap::new();
    for i in 0..n {
        if is_target(i, &mut fallen) {
            min_heap.push(Reverse((l - a[i], i)));
        }
    }

    let mut ans = 0;
    while let Some(Reverse((time, i))) = min_heap.pop() {
        ans = ans.max(time);
        fallen[i] = true;

        if i > 0 && !fallen[i - 1] && is_target(i - 1, &mut fallen) {
            min_heap.push(Reverse((l - a[i - 1] + time, i - 1)));
        }

        if i < n - 1 && !fallen[i + 1] && is_target(i + 1, &mut fallen) {
            min_heap.push(Reverse((l - a[i + 1] + time, i + 1)));
        }
    }

    println!("{}", ans);
}
