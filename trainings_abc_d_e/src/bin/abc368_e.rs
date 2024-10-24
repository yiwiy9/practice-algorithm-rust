use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const DEPART: usize = 1;
const ARRIVE: usize = 0;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        abst: [(Usize1,Usize1,usize,usize); m],
    }

    // イベントソートなので、出発と到着を別々のイベントとして扱う
    let mut event = BinaryHeap::new();
    for (i, &(_, _, s, t)) in abst.iter().enumerate() {
        // 同じ時間の場合は到着を先に処理する
        event.push(Reverse((s, DEPART, i)));
        event.push(Reverse((t, ARRIVE, i)));
    }

    let mut ans = vec![0; m];
    let mut last_train = vec![m; n];

    // 処理も出発と到着で別々の処理を行う
    while let Some(Reverse((time, event, i))) = event.pop() {
        let (a, b, _, _) = abst[i];
        match event {
            DEPART => {
                if i == 0 {
                    ans[0] = x;
                    continue;
                }
                let j = last_train[a];
                if j == m {
                    continue;
                }
                let last_tx = abst[j].3 + ans[j];
                ans[i] = if last_tx <= time { 0 } else { last_tx - time };
            }
            ARRIVE => {
                let j = last_train[b];
                if j == m {
                    last_train[b] = i;
                    continue;
                }
                let last_tx = abst[j].3 + ans[j];
                if time + ans[i] > last_tx {
                    last_train[b] = i;
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ans.iter().skip(1).join(" "));
}
