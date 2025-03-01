use proconio::{input, marker::Usize1};
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut bird = (0..n).collect::<Vec<_>>();
    let mut current_house = (0..n).collect::<Vec<_>>();
    let mut initial_house = (0..n).collect::<Vec<_>>();

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    a: Usize1,
                    b: Usize1,
                }
                let b_initial_house = current_house[b];
                bird[a] = b_initial_house;
            }
            2 => {
                input! {
                    a: Usize1,
                    b: Usize1,
                }
                let a_initial_house = current_house[a];
                let b_initial_house = current_house[b];

                let current_a_initial_house = initial_house[a_initial_house];
                let current_b_initial_house = initial_house[b_initial_house];

                current_house[a] = b_initial_house;
                current_house[b] = a_initial_house;
                initial_house[a_initial_house] = current_b_initial_house;
                initial_house[b_initial_house] = current_a_initial_house;
            }
            3 => {
                input! {
                    a: Usize1,
                }
                println!("{}", initial_house[bird[a]] + 1);
            }
            _ => unreachable!(),
        }
    }
}
