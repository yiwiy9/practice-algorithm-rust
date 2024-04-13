use itertools::Itertools as _;
use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
         a: [usize; n],
    }

    let mut b = a.clone();
    b.sort();

    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + b[i];
    }

    let mut ans = vec![];
    for a_i in a {
        let i = b.upper_bound(&a_i);
        if i == n {
            ans.push(0)
        } else {
            ans.push(s[n] - s[i]);
        }
    }

    println!("{}", ans.iter().join(" "))
}
