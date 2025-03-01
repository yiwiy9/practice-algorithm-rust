use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut field = vec![vec!['.'; n]; n];
    for i in 0..n {
        let j = n - 1 - i;
        for row in i..=j {
            for col in i..=j {
                field[row][col] = if i % 2 == 0 { '#' } else { '.' };
            }
        }
    }

    println!("{}", field.iter().map(|row| row.iter().join("")).join("\n"));
}
