use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        field: [Chars; h],
    }

    let mut ans = -1;
    for x in 0..h {
        for y in 0..w {
            let max = dfs(&field, h, w, &mut vec![vec![false; w]; h], (x, y), (x, y));
            if max >= 3 {
                ans = ans.max(max);
            }
        }
    }

    println!("{}", ans);
}

const DX: [i32; 4] = [1, 0, -1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

fn dfs(
    field: &Vec<Vec<char>>,
    h: usize,
    w: usize,
    seen: &mut Vec<Vec<bool>>,
    cur: (usize, usize),
    start: (usize, usize),
) -> i32 {
    let (x, y) = cur;
    if seen[x][y] {
        if cur == start {
            return 0;
        } else {
            return -1;
        }
    }

    if field[x][y] == '#' {
        return -1;
    }

    seen[x][y] = true;

    let mut max = -1;
    for dir in 0..4 {
        let nx = x as i32 + DX[dir];
        let ny = y as i32 + DY[dir];

        if nx < 0 || h as i32 <= nx || ny < 0 || w as i32 <= ny {
            continue;
        }

        let nx = nx as usize;
        let ny = ny as usize;

        max = max.max(dfs(field, h, w, seen, (nx, ny), start));
    }

    seen[x][y] = false;

    if max != -1 {
        max + 1
    } else {
        -1
    }
}
