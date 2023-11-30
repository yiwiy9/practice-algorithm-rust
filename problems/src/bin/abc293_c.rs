use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize;w]; h],
    }

    println!("{}", dfs(&a, (h - 1, w - 1), (0, 0), &mut vec![]));
}

pub fn dfs(
    field: &Vec<Vec<usize>>,
    goal: (usize, usize),
    cur: (usize, usize),
    path: &mut Vec<usize>,
) -> usize {
    let cur_val = field[cur.0][cur.1];

    if cur == goal {
        path.push(cur_val);
        if path.iter().all_unique() {
            path.pop();
            return 1;
        } else {
            path.pop();
            return 0;
        }
    }

    let mut sum = 0;
    if cur.0 < goal.0 {
        path.push(cur_val);
        sum += dfs(field, goal, (cur.0 + 1, cur.1), path);
        path.pop();
    }
    if cur.1 < goal.1 {
        path.push(cur_val);
        sum += dfs(field, goal, (cur.0, cur.1 + 1), path);
        path.pop();
    }
    sum
}
