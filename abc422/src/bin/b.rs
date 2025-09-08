use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                continue;
            }
            let mut cnt = 0;
            if i > 0 && s[i - 1][j] == '#' {
                cnt += 1;
            }
            if i < h - 1 && s[i + 1][j] == '#' {
                cnt += 1;
            }
            if j > 0 && s[i][j - 1] == '#' {
                cnt += 1;
            }
            if j < w - 1 && s[i][j + 1] == '#' {
                cnt += 1;
            }
            if cnt != 2 && cnt != 4 {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
