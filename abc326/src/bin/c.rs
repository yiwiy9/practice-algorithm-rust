use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }

    a.sort();

    let mut ans = 0;
    for (i, &a_i) in a.iter().enumerate() {
        let j = a.lower_bound(&(a_i + m));
        ans = ans.max(j - i);
    }

    println!("{}", ans);
}
