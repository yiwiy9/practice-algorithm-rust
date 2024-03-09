use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        q: usize,
    }

    let mut min_heap = BinaryHeap::new();

    let mut offset = 0;
    for _ in 0..q {
        input! {
            p: usize,
        }

        match p {
            1 => {
                input! {
                    x: i64,
                }
                min_heap.push(Reverse(x - offset));
            }
            2 => {
                input! {
                    x: i64,
                }
                offset += x;
            }
            3 => println!("{}", min_heap.pop().unwrap().0 + offset),
            _ => unreachable!(),
        }
    }
}
