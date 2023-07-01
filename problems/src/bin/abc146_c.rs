use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        a: i64,
        b: i64,
        x: i64,
    }

    let mut ans = 0;
    for d in 1..=18 {
        let n = (x - (b * d)) / a;
        if n <= 0 {
            break;
        }

        let mut num = n;
        let mut num_d = 0;
        while num != 0 {
            num /= 10;
            num_d += 1;
        }

        match num_d.cmp(&d) {
            Ordering::Greater => ans = 10_i64.pow(d as u32) - 1,
            Ordering::Less => break,
            Ordering::Equal => ans = n,
        }
    }

    println!("{}", ans.min(1_000_000_000));
}
