use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 8],
    }

    let mut row = vec![false; 8];
    let mut col = vec![false; 8];
    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == '#' {
                row[i] = true;
                col[j] = true;
            }
        }
    }

    let mut cnt = 0;
    for i in 0..8 {
        for j in 0..8 {
            if !row[i] && !col[j] {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt);
}
