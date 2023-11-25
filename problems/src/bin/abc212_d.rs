use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        q: usize,
    }

    let mut heap = BinaryHeap::new();

    let mut adding_num = 0;
    for _ in 0..q {
        input! {
            op: usize,
        }

        match op {
            1 => {
                input! {
                    x: i64,
                }
                heap.push(Reverse(x - adding_num));
            }
            2 => {
                input! {
                    x: i64,
                }
                adding_num += x;
            }
            3 => println!("{}", heap.pop().unwrap().0 + adding_num),
            _ => unreachable!(),
        }
    }
}
