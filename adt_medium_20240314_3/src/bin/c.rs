use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let mut ans = vec![vec![]; w];
    for i in 0..h {
        for j in 0..w {
            ans[j].push(a[i][j]);
        }
    }

    println!("{}", ans.iter().map(|x| x.iter().join(" ")).join("\n"));
}
