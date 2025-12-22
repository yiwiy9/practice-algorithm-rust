use ac_library::modint::ModInt998244353 as Mint;
use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        mut b: [usize; m],
    }

    b.sort();

    let mut b_s = vec![0; m + 1];
    for i in 0..m {
        b_s[i + 1] = b_s[i] + b[i];
    }

    let mut ans = Mint::new(0);
    for &a_i in &a {
        let upper_i = b.upper_bound(&a_i);
        let left = upper_i * a_i - b_s[upper_i];
        let right = b_s[m] - b_s[upper_i] - (m - upper_i) * a_i;
        ans += Mint::new(left) + Mint::new(right);
    }

    println!("{}", ans);
}
