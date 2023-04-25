use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
    }

    let mut heap = BinaryHeap::new();
    for &a_i in &a {
        heap.push(Reverse(a_i));
    }

    let mut ordered_price = Vec::<i64>::new();
    while ordered_price.len() < k {
        if let Some(Reverse(min_price)) = heap.pop() {
            if !ordered_price.is_empty() && min_price == *ordered_price.last().unwrap() {
                continue;
            }

            ordered_price.push(min_price);
            for &a_i in &a {
                heap.push(Reverse(min_price + a_i));
            }
        }
    }

    println!("{}", ordered_price.last().unwrap());
}
