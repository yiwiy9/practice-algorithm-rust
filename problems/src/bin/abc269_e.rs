use proconio::{input, source::line::LineSource};
use std::io::{stdin, stdout, BufReader, Write};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        n: usize,
    }

    let mut row_left = 1;
    let mut row_right = n + 1;
    while row_right - row_left > 1 {
        let mid = (row_right + row_left) / 2;

        println!("? {} {} {} {}", row_left, mid - 1, 1, n);
        stdout().flush().unwrap();

        input! {
            from &mut source,
            mut cnt: usize,
        }

        if cnt == mid - row_left {
            row_left = mid;
        } else {
            row_right = mid;
        }
    }

    let mut column_left = 1;
    let mut column_right = n + 1;
    while column_right - column_left > 1 {
        let mid = (column_right + column_left) / 2;

        println!("? {} {} {} {}", 1, n, column_left, mid - 1);
        stdout().flush().unwrap();

        input! {
            from &mut source,
            mut cnt: usize,
        }

        if cnt == mid - column_left {
            column_left = mid;
        } else {
            column_right = mid;
        }
    }

    println!("! {} {}", row_left, column_left);
}
