use itertools::Itertools as _;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }

    let mut b = vec![vec!['.'; n]; n];
    for i in 0..n / 2 {
        for x in i..n - i {
            let left_xy = (x, i);
            let converted_left_xy = calc(n, i, left_xy);
            b[converted_left_xy.0][converted_left_xy.1] = a[left_xy.0][left_xy.1];

            let right_xy = (x, n - 1 - i);
            let converted_right_xy = calc(n, i, right_xy);
            b[converted_right_xy.0][converted_right_xy.1] = a[right_xy.0][right_xy.1];
        }
        for y in i + 1..n - 1 - i {
            let top_xy = (i, y);
            let converted_top_xy = calc(n, i, top_xy);
            b[converted_top_xy.0][converted_top_xy.1] = a[top_xy.0][top_xy.1];

            let bottom_xy = (n - 1 - i, y);
            let converted_bottom_xy = calc(n, i, bottom_xy);
            b[converted_bottom_xy.0][converted_bottom_xy.1] = a[bottom_xy.0][bottom_xy.1];
        }
    }

    println!("{}", b.iter().map(|row| row.iter().join("")).join("\n"));
}

fn calc(n: usize, i: usize, xy: (usize, usize)) -> (usize, usize) {
    let (x, y) = xy;
    match (i + 1) % 4 {
        0 => (x, y),
        1 => (y, n - 1 - x),
        2 => (n - 1 - x, n - 1 - y),
        3 => (n - 1 - y, x),
        _ => unreachable!(),
    }
}
