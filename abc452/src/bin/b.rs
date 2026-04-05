use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }

    let mut ans = vec![vec!['.'; w]; h];
    for i in 0..h {
        for j in 0..w {
            if i == 0 || i == h - 1 || j == 0 || j == w - 1 {
                ans[i][j] = '#';
            }
        }
    }

    println!("{}", ans.iter().map(|row| row.iter().join("")).join("\n"));
}
