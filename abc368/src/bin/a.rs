use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut ans = vec![];
    for i in n - k..n {
        ans.push(a[i]);
    }
    for i in 0..n - k {
        ans.push(a[i]);
    }
    println!("{}", ans.iter().join(" "));
}
