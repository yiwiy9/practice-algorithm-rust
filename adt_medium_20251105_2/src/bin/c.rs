use itertools::Itertools;
use proconio::input;

const DX: [i32; 4] = [-1, 0, 1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    }

    let mut field = vec![vec!['.'; w]; h];
    let mut i = 0;
    let mut j = 0;
    let mut dir = 0;
    for _ in 0..n {
        if field[i][j] == '.' {
            field[i][j] = '#';
            dir += 1;
            dir %= 4;
        } else {
            field[i][j] = '.';
            dir += 3;
            dir %= 4;
        }
        i = (i as i32 + DX[dir] + h as i32) as usize % h;
        j = (j as i32 + DY[dir] + w as i32) as usize % w;
    }

    println!("{}", field.iter().map(|row| row.iter().join("")).join("\n"));
}
