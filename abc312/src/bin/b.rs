use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    for i in 0..n {
        if i + 9 > n {
            continue;
        }
        for j in 0..m {
            if j + 9 > m {
                continue;
            }
            let mut ok = true;
            for d_i in 0..9 {
                for d_j in 0..9 {
                    if d_i < 3 && d_j < 3 && s[i + d_i][j + d_j] != '#' {
                        ok = false
                    }
                    if d_i > 5 && d_j > 5 && s[i + d_i][j + d_j] != '#' {
                        ok = false
                    }
                    if ((d_i == 3 && d_j <= 3)
                        || (d_i <= 3 && d_j == 3)
                        || (d_i == 5 && d_j >= 5)
                        || (d_i >= 5 && d_j == 5))
                        && s[i + d_i][j + d_j] != '.'
                    {
                        ok = false
                    }
                }
            }
            if ok {
                println!("{} {}", i + 1, j + 1);
            }
        }
    }
}
