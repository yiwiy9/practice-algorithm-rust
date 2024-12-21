use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut ans = vec![0; 2 * n + 1];
    for i in 0..n {
        let parent = a[i];
        ans[2 * i + 1] = ans[parent] + 1;
        ans[2 * i + 2] = ans[parent] + 1;
    }

    println!("{}", ans.iter().join("\n"));
}
