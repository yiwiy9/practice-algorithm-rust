use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut bit_on_pos = vec![];
    for i in 0..60 {
        if n & (1 << i) != 0 {
            bit_on_pos.push(i);
        }
    }

    let mut ans = vec![];
    for bit in 0..(1 << bit_on_pos.len()) {
        let mut num = 0_usize;
        for i in 0..bit_on_pos.len() {
            if bit & (1 << i) != 0 {
                num += 1 << bit_on_pos[i];
            }
        }
        ans.push(num);
    }

    ans.sort();

    println!("{}", ans.iter().join("\n"));
}
