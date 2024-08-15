use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        q: usize,
    }

    let mut heap = BinaryHeap::new();
    let mut offset = 0;

    for _ in 0..q {
        input! {
            query: u8,
        }

        match query {
            1 => {
                input! {
                    x: i64,
                }
                heap.push(Reverse(x - offset));
            }
            2 => {
                input! {
                    x: i64,
                }
                offset += x;
            }
            3 => {
                let Reverse(x) = heap.pop().unwrap();
                println!("{}", x + offset);
            }
            _ => unreachable!(),
        }
    }
}
