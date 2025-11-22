use proconio::{input, marker::Chars};
use std::collections::VecDeque;
const INF: usize = 1 << 60;

fn main() {
    input! {
        _n: usize,
        a: Chars,
    }

    let mut a_deque = VecDeque::from_iter(a.iter().map(|&a_i| (a_i, 1)));
    while a_deque.len() > 1 {
        let mut new_a_deque = VecDeque::new();

        while a_deque.len() > 1 {
            let mut total_cost = 0;
            let mut max_cost = 0;
            let mut one_cnt = 0;
            let mut min_one_cost = INF;
            let mut min_zero_cost = INF;
            for _ in 0..3 {
                let (c, cost) = a_deque.pop_front().unwrap();
                total_cost += cost;
                max_cost = max_cost.max(cost);

                if c == '1' {
                    one_cnt += 1;
                    min_one_cost = min_one_cost.min(cost);
                } else {
                    min_zero_cost = min_zero_cost.min(cost);
                }
            }
            match one_cnt {
                0 => new_a_deque.push_back(('0', total_cost - max_cost)),
                1 => new_a_deque.push_back(('0', min_zero_cost)),
                2 => new_a_deque.push_back(('1', min_one_cost)),
                3 => new_a_deque.push_back(('1', total_cost - max_cost)),
                _ => unreachable!(),
            }
        }

        a_deque = new_a_deque;
    }

    println!("{}", a_deque[0].1);
}
