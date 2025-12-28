use proconio::{input, marker::Chars};

fn main() {
    input! {
        m: usize,
        n: usize,
        flag: [Chars; m],
        mark: [Chars; 2],
    }

    let is_mark = |f1: char, f2: char, f3: char, f4: char| -> bool {
        f1 == mark[0][0] && f2 == mark[0][1] && f3 == mark[1][0] && f4 == mark[1][1]
    };

    let mut base: usize = 0;
    for i in 0..m - 1 {
        for j in 0..n - 1 {
            if is_mark(
                flag[i][j],
                flag[i][j + 1],
                flag[i + 1][j],
                flag[i + 1][j + 1],
            ) {
                base += 1;
            }
        }
    }

    let get_points = |i: usize, j: usize, c: char| -> usize {
        let mut points = 0;
        if i > 0 && j > 0 && is_mark(flag[i - 1][j - 1], flag[i - 1][j], flag[i][j - 1], c) {
            points += 1;
        }
        if i > 0 && j + 1 < n && is_mark(flag[i - 1][j], flag[i - 1][j + 1], c, flag[i][j + 1]) {
            points += 1;
        }
        if i + 1 < m && j > 0 && is_mark(flag[i][j - 1], c, flag[i + 1][j - 1], flag[i + 1][j]) {
            points += 1;
        }
        if i + 1 < m && j + 1 < n && is_mark(c, flag[i][j + 1], flag[i + 1][j], flag[i + 1][j + 1])
        {
            points += 1;
        }
        points
    };

    let mut ans = base;
    for i in 0..m {
        for j in 0..n {
            let original = get_points(i, j, flag[i][j]);
            let mut max_points = original;
            max_points = max_points.max(get_points(i, j, 'J'));
            max_points = max_points.max(get_points(i, j, 'O'));
            max_points = max_points.max(get_points(i, j, 'I'));
            ans = ans.max(base + (max_points - original));
        }
    }

    println!("{}", ans);
}
