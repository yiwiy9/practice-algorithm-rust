use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut start = (0, 0);
    for (i, s_row) in s.iter().enumerate() {
        for (j, c) in s_row.iter().enumerate() {
            if *c == 'S' {
                start = (i, j);
            }
        }
    }

    let mut dist = vec![vec![-1; w]; h];
    dist[start.0][start.1] = 0;
    dfs(&s, h, w, &mut dist, start);

    println!(
        "{}",
        if dist[start.0][start.1] != 0 {
            "Yes"
        } else {
            "No"
        }
    );
}

const DX: [i32; 4] = [1, 0, -1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

fn dfs(field: &Vec<Vec<char>>, h: usize, w: usize, dist: &mut Vec<Vec<i32>>, cur: (usize, usize)) {
    let (x, y) = cur;

    for dir in 0..4 {
        let nx = x as i32 + DX[dir];
        let ny = y as i32 + DY[dir];

        if nx < 0 || h as i32 <= nx || ny < 0 || w as i32 <= ny {
            continue;
        }

        let nx = nx as usize;
        let ny = ny as usize;

        if field[nx][ny] == '#' {
            continue;
        }
        if field[nx][ny] != 'S' && dist[nx][ny] != -1 {
            continue;
        }
        if field[nx][ny] == 'S' && dist[x][y] < 3 {
            continue;
        }

        dist[nx][ny] = dist[x][y] + 1;
        dfs(field, h, w, dist, (nx, ny));
    }
}
