use std::convert::TryInto;

use proconio::input;

fn main() {
    input! {
        q: usize,
        ops: [(i32, i32); q],
    }

    let mut top_cards = Vec::new();
    let mut bottom_cards = Vec::new();

    for &op in &ops {
        match op {
            (1, x) => top_cards.push(x),
            (2, x) => bottom_cards.push(x),
            (3, x) => {
                let x_usize: usize = x.try_into().unwrap();
                if x_usize <= top_cards.len() {
                    println!("{}", top_cards[top_cards.len() - x_usize]);
                } else {
                    println!("{}", bottom_cards[x_usize - 1 - top_cards.len()]);
                }
            }
            _ => unreachable!(),
        }
    }
}
