use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        lr: [(Usize1, Usize1); q],
    }

    let mut acc = vec![0; n + 1];
    for i in 0..n - 1 {
        acc[i + 1] = acc[i] + if s[i] == s[i + 1] { 1 } else { 0 };
    }

    for (l, r) in lr {
        println!("{}", acc[r] - acc[l]);
    }
}
