use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        a: [usize; n],
    }

    let mut min_heap = BinaryHeap::new();
    for &a_i in &a {
        min_heap.push(Reverse(a_i));
    }

    let mut before_price: usize = 1 << 60;
    while let Some(Reverse(price)) = min_heap.pop() {
        if price == before_price {
            continue;
        }

        before_price = price;
        k -= 1;
        if k == 0 {
            println!("{}", price);
            return;
        }

        for &a_i in &a {
            min_heap.push(Reverse(a_i + price));
        }
    }
}
