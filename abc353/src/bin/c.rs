use proconio::input;
use superslice::*;

const MOD: usize = 100_000_000;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();

    let mut ans = (n - 1) * a.iter().sum::<usize>();
    for (i, &a_i) in a.iter().enumerate() {
        if a_i < MOD / 2 {
            let b = MOD - a_i;
            let j = a.lower_bound(&b);
            ans -= (n - j) * MOD;
        } else {
            ans -= (n - i - 1) * MOD;
        }
    }

    println!("{}", ans);
}
