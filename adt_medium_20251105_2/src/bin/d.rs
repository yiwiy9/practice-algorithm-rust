use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let keyboard = s.iter().map(|&c| c as usize - 'A' as usize).collect_vec();

    let mut ans = 0;
    let mut cur_pos = keyboard.iter().position(|&num| num == 0).unwrap() as isize;
    for i in 1..26 {
        let next_pos = keyboard.iter().position(|&num| num == i).unwrap() as isize;
        ans += (next_pos - cur_pos).abs();
        cur_pos = next_pos;
    }

    println!("{}", ans);
}
