use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut min_s = s.clone();
    let mut max_s = s.clone();
    for i in 1..s.len() {
        let mut cur_s = vec![];
        for j in 0..s.len() {
            cur_s.push(s[(i + j) % s.len()]);
        }

        if cur_s < min_s {
            min_s = cur_s.clone();
        }
        if cur_s > max_s {
            max_s = cur_s.clone();
        }
    }

    println!("{}", min_s.iter().join(""));
    println!("{}", max_s.iter().join(""));
}
