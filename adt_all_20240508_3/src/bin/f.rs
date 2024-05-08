use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        g: [Chars; h],
    }

    let ans = dfs(&g, &mut vec![vec![false; w]; h], (0, 0));

    if ans == (h, w) {
        println!("-1");
    } else {
        println!("{} {}", ans.0 + 1, ans.1 + 1);
    }
}

fn dfs(field: &Vec<Vec<char>>, seen: &mut Vec<Vec<bool>>, cur: (usize, usize)) -> (usize, usize) {
    let (x, y) = cur;
    if seen[x][y] {
        return (field.len(), field[0].len());
    }
    seen[x][y] = true;
    let dir = field[x][y];

    match dir {
        'U' => {
            if x == 0 {
                return (x, y);
            } else {
                return dfs(field, seen, (x - 1, y));
            }
        }
        'D' => {
            if x == field.len() - 1 {
                return (x, y);
            } else {
                return dfs(field, seen, (x + 1, y));
            }
        }
        'L' => {
            if y == 0 {
                return (x, y);
            } else {
                return dfs(field, seen, (x, y - 1));
            }
        }
        'R' => {
            if y == field[0].len() - 1 {
                return (x, y);
            } else {
                return dfs(field, seen, (x, y + 1));
            }
        }
        _ => unreachable!(),
    }
}
