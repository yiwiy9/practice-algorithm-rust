use ac_library::modint::ModInt998244353 as Mint;
use proconio::input;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        n: usize,
    }

    let mut dp = HashMap::new();
    let mut heap = BinaryHeap::new();
    dp.insert(n, Mint::new(1));
    for i in 1..=6 {
        if n % i == 0 {
            heap.push(n / i);
        }
    }
    while let Some(cur) = heap.pop() {
        if dp.contains_key(&cur) {
            continue;
        }

        let mut dp_sum = Mint::new(0);
        for i in 1..=6 {
            if let Some(&next) = dp.get(&(cur * i)) {
                dp_sum += next;
            }
        }
        dp.insert(cur, dp_sum / 5);

        for i in 1..=6 {
            if cur % i == 0 {
                heap.push(cur / i);
            }
        }
    }

    println!("{}", dp.get(&1).unwrap_or(&Mint::new(0)).val());
}
