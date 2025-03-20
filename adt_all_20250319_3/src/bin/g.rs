use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut back_map = HashMap::new();
    let mut front_map = HashMap::new();
    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    x: usize,
                    y: usize,
                }

                back_map.insert(x, y);
                front_map.insert(y, x);
            }
            2 => {
                input! {
                    x: usize,
                    y: usize,
                }

                back_map.remove(&x);
                front_map.remove(&y);
            }
            3 => {
                input! {
                    x: usize,
                }

                let mut front_vec = vec![x];
                let mut cur = x;
                while let Some(&front) = front_map.get(&cur) {
                    front_vec.push(front);
                    cur = front;
                }
                front_vec.reverse();

                let mut back_vec = vec![];
                let mut cur = x;
                while let Some(&back) = back_map.get(&cur) {
                    back_vec.push(back);
                    cur = back;
                }

                front_vec.extend(back_vec);
                println!("{} {}", front_vec.len(), front_vec.iter().join(" "));
            }
            _ => unreachable!(),
        }
    }
}
