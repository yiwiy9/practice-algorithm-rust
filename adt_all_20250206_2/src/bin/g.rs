use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut row_cnt = vec![0; n];
    let mut col_cnt = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'o' {
                row_cnt[i] += 1;
                col_cnt[j] += 1;
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'o' {
                ans += (row_cnt[i] - 1) * (col_cnt[j] - 1);
            }
        }
    }

    println!("{}", ans);
}
