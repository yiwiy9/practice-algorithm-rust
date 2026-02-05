use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

/// https://atcoder.jp/contests/abc443/tasks/abc443_e
/// https://atcoder.jp/contests/abc443/editorial/15178
/// WA の原因: 破壊できる壁は最初に決めうちできず、動的に決まっていく
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            c: Usize1,
            s: [Chars; n],
        }

        let mut col_wall_start = vec![n; n];
        for (i, row) in s.iter().enumerate() {
            for (j, &s_ij) in row.iter().enumerate() {
                if s_ij == '#' {
                    col_wall_start[j] = i;
                }
            }
        }

        let mut dp = vec![vec![0; n]; n];
        for i in 0..n {
            dp[i][c] = 1;
        }

        for i in (0..n - 1).rev() {
            for j in 0..n {
                let mut ok = false;
                if j > 0 && dp[i + 1][j - 1] == 1 {
                    ok = true;
                }
                if dp[i + 1][j] == 1 {
                    ok = true;
                }
                if j < n - 1 && dp[i + 1][j + 1] == 1 {
                    ok = true;
                }

                if !ok {
                    continue;
                }

                if s[i][j] == '.' {
                    dp[i][j] = 1
                } else if col_wall_start[j] == i {
                    for ii in 0..=i {
                        dp[ii][j] = 1;
                    }
                }
            }
        }

        println!("{}", dp[0].iter().join(""));
    }
}
