use itertools::Itertools as _;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let mut ans = vec![vec!['.'; w]; h];
    for (i, row) in a.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == 0 {
                ans[i][j] = '.';
            } else {
                ans[i][j] = (b'A' + (c - 1) as u8) as char;
            }
        }
    }

    println!("{}", ans.iter().map(|row| row.iter().join("")).join("\n"));
}
