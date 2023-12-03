use proconio::{input, marker::Chars};

/**
 * https://atcoder.jp/contests/abc331/tasks/abc331_d
 * https://atcoder.jp/contests/abc331/editorial/7822
 */
fn main() {
    input! {
        n: usize,
        q: usize,
        p: [Chars; n],
        abcd: [(usize,usize,usize,usize); q],
    }

    // 二次元累積和
    // https://qiita.com/drken/items/56a6b68edef8fc605821#4-%E4%BA%8C%E6%AC%A1%E5%85%83%E7%B4%AF%E7%A9%8D%E5%92%8C
    let mut s = vec![vec![0_i64; n + 1]; n + 1];
    for (i, row) in p.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            let cur = if c == 'B' { 1 } else { 0 };
            s[i + 1][j + 1] = s[i][j + 1] + s[i + 1][j] - s[i][j] + cur;
        }
    }

    let calc = |i: usize, j: usize| -> i64 {
        let n_row_cnt = (i / n) as i64;
        let n_column_cnt = (j / n) as i64;

        let mut sum = 0;
        sum += s[i % n][j % n];
        sum += s[n][n] * n_row_cnt * n_column_cnt;
        sum += s[i % n][n] * n_column_cnt;
        sum += s[n][j % n] * n_row_cnt;

        sum
    };

    for &(a, b, c, d) in &abcd {
        let cc = c + 1;
        let dd = d + 1;
        println!("{}", calc(cc, dd) - calc(a, dd) - calc(cc, b) + calc(a, b));
    }
}
