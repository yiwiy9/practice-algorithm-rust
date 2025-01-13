use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();
    let mut ans = 0;
    for i in 0..n {
        let idx: usize = a.lower_bound(&(a[i] * 2));
        ans += n - idx;
    }

    println!("{}", ans);
}
