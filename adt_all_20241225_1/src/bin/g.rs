use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        q: usize,
    }

    let mut min_heap = BinaryHeap::new();
    let mut base = 0;
    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    x: i64,
                }
                min_heap.push(Reverse(x - base));
            }
            2 => {
                input! {
                    x: i64,
                }
                base += x;
            }
            3 => {
                let Reverse(x) = min_heap.pop().unwrap();
                println!("{}", x + base);
            }
            _ => unreachable!(),
        }
    }
}
