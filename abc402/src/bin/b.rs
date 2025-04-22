use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut queue = VecDeque::new();

    for _ in 0..q {
        input! {
            t: usize,
        }

        if t == 1 {
            input! {
                x: usize,
            }
            queue.push_back(x);
        } else if t == 2 {
            println!("{}", queue.pop_front().unwrap());
        }
    }
}
