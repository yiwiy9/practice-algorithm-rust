use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    println!(
        "{}",
        if bfs(&s, h, w, &mut vec![vec![false; w]; h], (0, 0)) {
            "Yes"
        } else {
            "No"
        }
    );
}

const DX: [i32; 4] = [1, 0, -1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

fn bfs(
    field: &Vec<Vec<char>>,
    h: usize,
    w: usize,
    seen: &mut Vec<Vec<bool>>,
    cur: (usize, usize),
) -> bool {
    if cur == (h - 1, w - 1) {
        return true;
    }

    let (x, y) = cur;

    seen[x][y] = true;

    let mut ok = false;
    for dir in 0..4 {
        let nx = x as i32 + DX[dir];
        let ny = y as i32 + DY[dir];

        if nx < 0 || h as i32 <= nx || ny < 0 || w as i32 <= ny {
            continue;
        }

        let nx = nx as usize;
        let ny = ny as usize;

        if seen[nx][ny] {
            continue;
        }

        if (field[x][y] == 's' && field[nx][ny] == 'n')
            || (field[x][y] == 'n' && field[nx][ny] == 'u')
            || (field[x][y] == 'u' && field[nx][ny] == 'k')
            || (field[x][y] == 'k' && field[nx][ny] == 'e')
            || (field[x][y] == 'e' && field[nx][ny] == 's')
        {
            ok = bfs(field, h, w, seen, (nx, ny));
            if ok {
                break;
            }
        }
    }

    ok
}
