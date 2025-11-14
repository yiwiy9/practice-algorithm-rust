use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        c: [[usize; 3]; 3],
    }

    let mut black_list = vec![vec![]; 9];
    for i in 0..3 {
        for j in 0..3 {
            match j {
                0 => {
                    if c[i][j + 1] == c[i][j + 2] {
                        black_list[i * 3 + j].push((i * 3 + j + 1, i * 3 + j + 2));
                    }
                }
                1 => {
                    if c[i][j - 1] == c[i][j + 1] {
                        black_list[i * 3 + j].push((i * 3 + j - 1, i * 3 + j + 1));
                    }
                }
                2 => {
                    if c[i][j - 2] == c[i][j - 1] {
                        black_list[i * 3 + j].push((i * 3 + j - 2, i * 3 + j - 1));
                    }
                }
                _ => unreachable!(),
            }

            match i {
                0 => {
                    if c[i + 1][j] == c[i + 2][j] {
                        black_list[i * 3 + j].push(((i + 1) * 3 + j, (i + 2) * 3 + j));
                    }
                }
                1 => {
                    if c[i - 1][j] == c[i + 1][j] {
                        black_list[i * 3 + j].push(((i - 1) * 3 + j, (i + 1) * 3 + j));
                    }
                }
                2 => {
                    if c[i - 2][j] == c[i - 1][j] {
                        black_list[i * 3 + j].push(((i - 2) * 3 + j, (i - 1) * 3 + j));
                    }
                }
                _ => unreachable!(),
            }

            if i == j {
                match i {
                    0 => {
                        if c[i + 1][j + 1] == c[i + 2][j + 2] {
                            black_list[i * 3 + j].push(((i + 1) * 3 + j + 1, (i + 2) * 3 + j + 2));
                        }
                    }
                    1 => {
                        if c[i - 1][j - 1] == c[i + 1][j + 1] {
                            black_list[i * 3 + j].push(((i - 1) * 3 + j - 1, (i + 1) * 3 + j + 1));
                        }
                    }
                    2 => {
                        if c[i - 2][j - 2] == c[i - 1][j - 1] {
                            black_list[i * 3 + j].push(((i - 2) * 3 + j - 2, (i - 1) * 3 + j - 1));
                        }
                    }
                    _ => unreachable!(),
                }
            }
            if i + j == 2 {
                match i {
                    0 => {
                        if c[i + 1][j - 1] == c[i + 2][j - 2] {
                            black_list[i * 3 + j].push(((i + 1) * 3 + j - 1, (i + 2) * 3 + j - 2));
                        }
                    }
                    1 => {
                        if c[i - 1][j + 1] == c[i + 1][j - 1] {
                            black_list[i * 3 + j].push(((i - 1) * 3 + j + 1, (i + 1) * 3 + j - 1));
                        }
                    }
                    2 => {
                        if c[i - 2][j + 2] == c[i - 1][j + 1] {
                            black_list[i * 3 + j].push(((i - 2) * 3 + j + 2, (i - 1) * 3 + j + 1));
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    let mut total = 0;
    let mut cnt = 0;
    for order in (0..9).permutations(9) {
        let mut ok = true;
        let mut seen = HashSet::new();
        for &i in &order {
            for (u, v) in &black_list[i] {
                if seen.contains(u) && seen.contains(v) {
                    ok = false;
                }
            }
            seen.insert(i);
        }
        if ok {
            cnt += 1;
        }
        total += 1;
    }

    println!("{:.10}", cnt as f64 / total as f64);
}
