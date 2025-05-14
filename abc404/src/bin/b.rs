use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
        t: [Chars; n],
    }

    let mut ans = n * n + 5;

    for i in 0..4 {
        let mut cnt = i;
        for j in 0..n {
            for k in 0..n {
                if s[j][k] != t[j][k] {
                    cnt += 1;
                }
            }
        }
        ans = ans.min(cnt);
        s = rotate_grid(&s);
    }

    println!("{}", ans);
}

pub fn rotate_grid(field: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = field.len();
    let m = field[0].len();
    let mut result = vec![vec!['.'; n]; m];
    for (i, row) in field.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            result[j][n - 1 - i] = c;
        }
    }
    result
}
