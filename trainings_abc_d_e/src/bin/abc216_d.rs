use proconio::{input, marker::Usize1};
use std::collections::{HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut cylinders = vec![VecDeque::new(); m];
    let mut colors = vec![vec![]; n];
    for i in 0..m {
        input! {
            k: usize,
        }
        for _ in 0..k {
            input! {
                a: Usize1,
            }
            cylinders[i].push_back(a);
            colors[a].push(i);
        }
    }

    let mut top_of_cylinders = HashSet::new();
    let mut can_pop = VecDeque::new();
    for i in 0..m {
        match cylinders[i].pop_front() {
            Some(a) => {
                if top_of_cylinders.contains(&a) {
                    top_of_cylinders.remove(&a);
                    can_pop.push_back(a);
                } else {
                    top_of_cylinders.insert(a);
                }
            }
            None => unreachable!(),
        }
    }

    while let Some(a) = can_pop.pop_front() {
        for &i in &colors[a] {
            if let Some(b) = cylinders[i].pop_front() {
                if top_of_cylinders.contains(&b) {
                    top_of_cylinders.remove(&b);
                    can_pop.push_back(b);
                } else {
                    top_of_cylinders.insert(b);
                }
            }
        }
    }

    println!(
        "{}",
        if cylinders.iter().all(|c| c.is_empty()) {
            "Yes"
        } else {
            "No"
        }
    );
}
