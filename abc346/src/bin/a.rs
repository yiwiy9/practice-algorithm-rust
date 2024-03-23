use itertools::Itertools as _;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = vec![];
    for i in 0..n - 1 {
        ans.push(a[i] * a[i + 1]);
    }

    println!("{}", ans.iter().join(" "));
}
