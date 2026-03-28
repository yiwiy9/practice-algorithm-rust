use proconio::input;
use std::collections::BTreeSet;

const DX: [i32; 4] = [1, 0, -1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

fn main() {
    input! {
        h: usize,
        w: usize,
        m: [[usize; w]; h],
    }

    let mut one = vec![vec![(0, 0); w]; h];

    for x in 0..h {
        for y in 0..w {
            dfs(&m, h, w, &mut one, x, y);
        }
    }

    let mut ans = 0;
    for row in &one {
        for &(_, second) in row {
            if second != 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}

pub fn dfs(
    field: &Vec<Vec<usize>>,
    h: usize,
    w: usize,
    one: &mut Vec<Vec<(usize, usize)>>,
    x: usize,
    y: usize,
) -> (usize, usize) {
    if one[x][y] != (0, 0) {
        return one[x][y];
    }

    let mut set = BTreeSet::new();
    for dir in 0..4 {
        let nx = x as i32 + DX[dir];
        let ny = y as i32 + DY[dir];
        if nx < 0 || h as i32 <= nx || ny < 0 || w as i32 <= ny {
            continue;
        }
        let nx = nx as usize;
        let ny = ny as usize;
        if field[nx][ny] > field[x][y] {
            continue;
        }

        let (res_first, res_second) = dfs(field, h, w, one, nx, ny);
        set.insert(res_first);
        set.insert(res_second);
    }

    let mut first = 0;
    let mut second = 0;
    for &num in &set {
        if num == 0 {
            continue;
        }

        if first == 0 {
            first = num;
            continue;
        }

        second = num;
        break;
    }

    if first == 0 {
        first = field[x][y];
    }

    let res = (first, second);
    one[x][y] = res;
    res
}
