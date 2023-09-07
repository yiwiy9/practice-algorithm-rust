use proconio::input;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

fn main() {
    input! {
        q: usize,
    }

    let mut deque = VecDeque::new();
    let mut heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new();

    for _ in 0..q {
        input! {
            op: usize,
        }

        match op {
            1 => {
                input! {
                    x: usize,
                }
                deque.push_back(x);
            }
            2 => println!(
                "{}",
                if heap.is_empty() {
                    deque.pop_front().unwrap()
                } else {
                    heap.pop().unwrap().0
                }
            ),
            3 => {
                while let Some(x) = deque.pop_front() {
                    heap.push(Reverse(x));
                }
            }
            _ => unreachable!(),
        }
    }
}
