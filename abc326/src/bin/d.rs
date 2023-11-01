use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        r: Chars,
        c: Chars,
    }

    let mut row_base = vec!['A', 'B', 'C'];
    for _ in 3..n {
        row_base.push('.');
    }

    let mut first_char_rows = vec![vec![]; 3];

    // next_permutation()
    permutohedron::heap_recursive(&mut row_base, |permutation| {
        let row: Vec<char> = permutation.to_vec();
        for &c in &row {
            if c != '.' {
                first_char_rows[c as usize - 'A' as usize].push(row.clone());
                break;
            }
        }
    });

    let mut grid = vec![];

    if rec(n, &r, &c, &first_char_rows, &mut grid, 0) {
        println!("Yes");
        println!("{}", grid.iter().map(|row| row.iter().join("")).join("\n"))
    } else {
        println!("No");
    }
}

fn rec(
    n: usize,
    r: &Vec<char>,
    c: &Vec<char>,
    first_char_rows: &Vec<Vec<Vec<char>>>,
    grid: &mut Vec<Vec<char>>,
    i: usize,
) -> bool {
    if i == n {
        return true;
    }

    for row in &first_char_rows[r[i] as usize - 'A' as usize] {
        grid.push(row.clone());
        if check_lines(n, c, grid) && rec(n, r, c, first_char_rows, grid, i + 1) {
            return true;
        }
        grid.pop();
    }

    false
}

fn check_lines(n: usize, c: &[char], grid: &Vec<Vec<char>>) -> bool {
    let mut ok = true;
    'parent: for (j, &c_j) in c.iter().enumerate() {
        for row in grid {
            if row[j] == '.' {
                continue;
            } else if row[j] == c_j {
                break;
            } else {
                ok = false;
                break 'parent;
            }
        }
    }
    if !ok {
        return false;
    }

    let mut abc_cnt = vec![vec![0; 3]; n];
    for row in grid {
        for (j, &c) in row.iter().enumerate() {
            if c != '.' {
                abc_cnt[j][c as usize - 'A' as usize] += 1;
            }
        }
    }

    if abc_cnt.iter().any(|line| line.iter().any(|&cnt| cnt > 1)) {
        return false;
    }

    if grid.len() == n {
        return abc_cnt.iter().all(|line| line.iter().all(|&cnt| cnt == 1));
    }

    true
}
