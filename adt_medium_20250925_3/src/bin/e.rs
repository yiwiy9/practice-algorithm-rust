use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
        t: [usize; n],
    }

    let mut ans = vec![1 << 60; n];
    let mut fast_stone = 1 << 60;
    for i in 0..2 * n {
        let i = i % n;
        ans[i] = t[i].min(fast_stone);
        fast_stone = ans[i] + s[i];
    }

    println!("{}", ans.iter().join("\n"));
}
