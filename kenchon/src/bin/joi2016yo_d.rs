use proconio::{input, marker::Usize1};
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        t: i64,
        q: usize,
        a: [(i64,u8); n],
        x: [Usize1; q],
    }

    let a_right = a
        .iter()
        .filter(|&&(_, d)| d == 1)
        .map(|&(a_i, _)| a_i)
        .collect::<BTreeSet<i64>>();
    let a_left = a
        .iter()
        .filter(|&&(_, d)| d == 2)
        .map(|&(a_i, _)| a_i)
        .collect::<BTreeSet<i64>>();

    for &x_i in &x {
        let (a_i, d_i) = a[x_i];

        if d_i == 1 {
            if let Some(&first_meet) = a_left.range(a_i..).next() {
                let first_meet_first_meet = *a_right.range(..first_meet).next_back().unwrap();
                let terminate = (first_meet + first_meet_first_meet) / 2;
                println!("{}", (a_i + t).min(terminate));
            } else {
                println!("{}", a_i + t);
            }
        } else if let Some(&first_meet) = a_right.range(..a_i).next_back() {
            let first_meet_first_meet = *a_left.range(first_meet..).next().unwrap();
            let terminate = (first_meet + first_meet_first_meet) / 2;
            println!("{}", (a_i - t).max(terminate));
        } else {
            println!("{}", a_i - t);
        }
    }
}
