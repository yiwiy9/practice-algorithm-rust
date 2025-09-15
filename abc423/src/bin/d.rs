use itertools::Itertools;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        k: usize,
        abc: [(usize, usize, usize); n],
    }

    let mut waiting = BinaryHeap::new();
    for (i, &(a, b, c)) in abc.iter().enumerate() {
        waiting.push(Reverse((a, b, c, i)));
    }
    let mut eating = BinaryHeap::new();

    let mut last_time = 0;
    let mut cur = 0;
    let mut ans = vec![0; n];
    while let Some(Reverse((a, b, c, i))) = waiting.pop() {
        last_time = last_time.max(a);

        while let Some(Reverse((end_time, end_c))) = eating.peek() {
            if *end_time <= last_time {
                cur -= end_c;
                eating.pop();
            } else {
                break;
            }
        }

        if cur + c <= k {
            cur += c;
            ans[i] = last_time;
            eating.push(Reverse((last_time + b, c)));
            continue;
        }

        while let Some(Reverse((end_time, end_c))) = eating.pop() {
            cur -= end_c;
            last_time = last_time.max(end_time);
            if cur + c <= k {
                cur += c;
                ans[i] = last_time;
                eating.push(Reverse((last_time + b, c)));
                break;
            }
        }
    }

    println!("{}", ans.iter().join("\n"));
}
