use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let res = rec(n);

    println!("{}", res.iter().map(|row| row.iter().join("")).join("\n"));
}

fn rec(n: usize) -> Vec<Vec<char>> {
    if n == 0 {
        return vec![vec!['#']];
    }

    let surround = rec(n - 1);
    let min_size = surround[0].len();
    let size: usize = min_size * 3;

    let mut res = vec![vec!['.'; size]; size];
    for i in 0..size {
        for j in 0..size {
            if i / min_size == 1 && j / min_size == 1 {
                continue;
            }
            res[i][j] = surround[i % min_size][j % min_size];
        }
    }

    res
}
