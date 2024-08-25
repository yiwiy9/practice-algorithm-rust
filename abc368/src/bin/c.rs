use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i64; n],
    }

    let mut left = VecDeque::new();
    let mut t = 0;
    for &h_i in &h {
        let mut h_mut = h_i;
        while !left.is_empty() && h_mut > 0 {
            h_mut -= left.pop_front().unwrap();
        }
        if h_mut <= 0 {
            continue;
        }
        match h_mut % 5 {
            1 => {
                t += (h_mut / 5 + 1) * 3;
                left.push_back(1);
                left.push_back(3);
            }
            2 => {
                t += (h_mut / 5 + 1) * 3;
                left.push_back(3);
            }
            0 => t += h_mut / 5 * 3,
            _ => t += (h_mut / 5 + 1) * 3,
        }
    }

    println!("{}", t - left.len() as i64);
}
