use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut row_cnt = vec![0; n];
    let mut column_cnt = vec![0; n];
    for (i, s_row) in s.iter().enumerate() {
        for (j, &c) in s_row.iter().enumerate() {
            if c == 'o' {
                row_cnt[i] += 1;
                column_cnt[j] += 1;
            }
        }
    }

    let mut ans: usize = 0;
    for (i, s_row) in s.iter().enumerate() {
        for (j, &c) in s_row.iter().enumerate() {
            if c == 'o' {
                ans += (row_cnt[i] - 1) * (column_cnt[j] - 1);
            }
        }
    }

    println!("{}", ans);
}
