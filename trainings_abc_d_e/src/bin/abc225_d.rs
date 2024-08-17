use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        _: usize,
        q: usize,
    }

    // 連結リストをHashMapで簡易的に実現する
    let mut prev_map = HashMap::new();
    let mut next_map = HashMap::new();

    for _ in 0..q {
        input! {
            op: u8,
        }

        match op {
            1 => {
                input! {
                    x: usize,
                    y: usize,
                }
                prev_map.insert(y, x);
                next_map.insert(x, y);
            }
            2 => {
                input! {
                    x: usize,
                    y: usize,
                }
                prev_map.remove(&y);
                next_map.remove(&x);
            }
            3 => {
                input! {
                    x: usize,
                }

                let mut before_x_vec = vec![];
                let mut cur_x = x;
                while let Some(&before_x) = prev_map.get(&cur_x) {
                    before_x_vec.push(before_x);
                    cur_x = before_x;
                }

                let mut after_x_vec = vec![];
                let mut cur_x = x;
                while let Some(&after_x) = next_map.get(&cur_x) {
                    after_x_vec.push(after_x);
                    cur_x = after_x;
                }

                before_x_vec.reverse();
                before_x_vec.push(x);
                before_x_vec.append(&mut after_x_vec);
                print!("{} ", before_x_vec.len());
                println!("{}", before_x_vec.iter().join(" "));
            }
            _ => unreachable!(),
        }
    }
}
