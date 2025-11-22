use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
        b: [[usize; w]; h],
    }

    let is_same = |field: &[Vec<usize>]| -> bool {
        field
            .iter()
            .enumerate()
            .all(|(i, row)| row.iter().enumerate().all(|(j, &c)| c == b[i][j]))
    };

    let mut que = std::collections::VecDeque::new();
    let mut seen = HashSet::new();
    seen.insert(a.clone());
    que.push_back((a, 0));
    while let Some((filed, dist)) = que.pop_front() {
        if is_same(&filed) {
            println!("{}", dist);
            return;
        }

        for i in 0..h - 1 {
            let mut next_field = filed.clone();
            next_field.swap(i, i + 1);

            if seen.contains(&next_field) {
                continue;
            }
            seen.insert(next_field.clone());
            que.push_back((next_field, dist + 1));
        }

        for j in 0..w - 1 {
            let mut next_field = filed.clone();
            next_field.iter_mut().for_each(|row| row.swap(j, j + 1));

            if seen.contains(&next_field) {
                continue;
            }
            seen.insert(next_field.clone());
            que.push_back((next_field, dist + 1));
        }
    }

    println!("-1");
}
