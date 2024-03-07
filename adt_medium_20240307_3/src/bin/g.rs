use std::mem;

use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
    }

    let mut ans = 0;
    while b != 0 {
        let div_cnt = a / b;
        ans += div_cnt;
        a %= b;
        mem::swap(&mut a, &mut b);
    }

    println!("{}", ans - 1);
}
