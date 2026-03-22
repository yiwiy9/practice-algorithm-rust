use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }

    let mut ans = vec![];
    while n > 0 {
        ans.push(n);
        n -= 1;
    }

    println!("{}", ans.iter().join(","));
}
