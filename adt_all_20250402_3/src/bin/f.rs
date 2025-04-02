use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    for &a_i in &a {
        let i = a.lower_bound(&(a_i * 2));
        ans += n - i;
    }

    println!("{}", ans);
}
