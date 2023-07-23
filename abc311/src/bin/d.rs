use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    let mut seen = vec![vec![false; m]; n];
    let mut seen_dir = vec![vec![vec![false; 4]; m]; n];
    seen[1][1] = true;
    dfs(&s, &mut seen, &mut seen_dir, (1, 1));

    let mut ans = 0;
    for row in &seen {
        for &v in row {
            if v {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

const DX: [i32; 4] = [1, 0, -1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

fn dfs(
    field: &Vec<Vec<char>>,
    seen: &mut Vec<Vec<bool>>,
    seen_dir: &mut Vec<Vec<Vec<bool>>>,
    cur: (usize, usize),
) {
    let (x, y) = cur;

    for dir in 0..4 {
        let mut nx = (x as i32 + DX[dir]) as usize;
        let mut ny = (y as i32 + DY[dir]) as usize;

        if field[nx][ny] == '#' {
            continue;
        }
        if seen_dir[nx][ny][dir] {
            continue;
        }

        seen[nx][ny] = true;
        seen_dir[nx][ny][dir] = true;
        loop {
            let nnx = (nx as i32 + DX[dir]) as usize;
            let nny = (ny as i32 + DY[dir]) as usize;
            if field[nnx][nny] == '#' {
                break;
            }
            nx = nnx;
            ny = nny;
            seen[nx][ny] = true;
            seen_dir[nx][ny][dir] = true;
            seen_dir[nx][ny][(dir + 2) % 4] = true;
        }

        dfs(field, seen, seen_dir, (nx, ny));
    }
}
