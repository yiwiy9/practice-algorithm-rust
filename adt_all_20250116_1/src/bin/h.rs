use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        m: usize,
        x_1: usize,
        abst: [(Usize1, Usize1, usize, usize); m],
    }

    let mut event = BinaryHeap::new();
    for (i, &(_, _, s, t)) in abst.iter().enumerate() {
        event.push(Reverse((s, 1, i)));
        event.push(Reverse((t, 0, i)));
    }

    let mut x = vec![0; m];
    x[0] = x_1;

    let mut last_arrived_time = vec![0; n];
    while let Some(Reverse((time, is_departure, i))) = event.pop() {
        if is_departure == 1 {
            let (a, _, _, _) = abst[i];
            if time < last_arrived_time[a] {
                x[i] = last_arrived_time[a] - time;
            }
        } else {
            let (_, b, _, _) = abst[i];
            last_arrived_time[b] = last_arrived_time[b].max(time + x[i]);
        }
    }

    println!("{}", x.iter().skip(1).join(" "));
}
