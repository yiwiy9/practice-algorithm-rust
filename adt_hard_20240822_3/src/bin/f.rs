use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        queries: [(u8,usize); q],
    }

    let mut x_sum = 0;
    for &(query_type, x) in &queries {
        match query_type {
            1 => {
                x_sum += x;
                x_sum %= n;
            }
            2 => {
                if x_sum < x {
                    println!("{}", s[x - x_sum - 1]);
                } else {
                    println!("{}", s[x + n - x_sum - 1]);
                }
            }
            _ => unreachable!(),
        }
    }
}
