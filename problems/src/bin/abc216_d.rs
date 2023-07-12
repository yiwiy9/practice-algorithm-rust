use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut deqs = vec![];
    let mut popped_pos = vec![vec![]; n];
    for j in 0..m {
        input! {
            k: usize,
            a: [Usize1; k],
        }
        let deq = VecDeque::from(a);
        popped_pos[*deq.front().unwrap()].push(j);
        deqs.push(deq);
    }

    let mut que = VecDeque::new();
    for (num, pos_vec) in popped_pos.iter().enumerate() {
        if pos_vec.len() == 2 {
            que.push_back(num);
        }
    }

    while let Some(num) = que.pop_front() {
        let pos_vec = popped_pos[num].clone();
        for &pos in &pos_vec {
            deqs[pos].pop_front();
            if deqs[pos].is_empty() {
                continue;
            }

            popped_pos[*deqs[pos].front().unwrap()].push(pos);
            if popped_pos[*deqs[pos].front().unwrap()].len() == 2 {
                que.push_back(*deqs[pos].front().unwrap());
            }
        }
    }

    println!(
        "{}",
        if popped_pos.iter().all(|pos_vec| pos_vec.len() == 2) {
            "Yes"
        } else {
            "No"
        }
    );
}
