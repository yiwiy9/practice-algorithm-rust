use itertools::Itertools as _;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }

    let mut ans = vec![vec!['0'; n]; n];
    for (i, a_row) in a.iter().enumerate() {
        for (j, num) in a_row.iter().enumerate() {
            if i == 0 {
                if j == 0 {
                    ans[i][j] = a[i + 1][j];
                } else {
                    ans[i][j] = a[i][j - 1];
                }
                continue;
            }
            if i == n - 1 {
                if j == n - 1 {
                    ans[i][j] = a[i - 1][j];
                } else {
                    ans[i][j] = a[i][j + 1];
                }
                continue;
            }
            if j == 0 {
                ans[i][j] = a[i + 1][j];
                continue;
            }
            if j == n - 1 {
                ans[i][j] = a[i - 1][j];
                continue;
            }
            ans[i][j] = *num;
        }
    }

    println!("{}", ans.iter().map(|row| row.iter().join("")).join("\n"));
}
