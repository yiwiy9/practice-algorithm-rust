use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut min_row = h;
    let mut max_row = 0;
    let mut min_column = w;
    let mut max_column = 0;

    for (i, row) in s.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == '#' {
                min_row = min_row.min(i);
                max_row = max_row.max(i);
                min_column = min_column.min(j);
                max_column = max_column.max(j);
            }
        }
    }

    for i in min_row..=max_row {
        for j in min_column..=max_column {
            if s[i][j] == '.' {
                println!("{} {}", i + 1, j + 1)
            }
        }
    }
}
