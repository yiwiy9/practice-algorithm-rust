use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 9],
    }

    let mut ans = 0;
    for i in 0..9 {
        for j in 0..9 {
            if s[i][j] == '.' {
                continue;
            }
            for ii in i..9 {
                for jj in j + 1..9 {
                    if s[ii][jj] == '.' {
                        continue;
                    }

                    let diff_i = ii as i32 - i as i32;
                    let diff_j = jj as i32 - j as i32;

                    let iii = ii as i32 + diff_j;
                    let jjj = jj as i32 - diff_i;
                    if iii < 0 || iii >= 9 || jjj < 0 || jjj >= 9 {
                        continue;
                    }
                    if s[iii as usize][jjj as usize] == '.' {
                        continue;
                    }

                    let iiii = iii - diff_i;
                    let jjjj = jjj - diff_j;
                    if iiii < 0 || iiii >= 9 || jjjj < 0 || jjjj >= 9 {
                        continue;
                    }
                    if s[iiii as usize][jjjj as usize] == '.' {
                        continue;
                    }

                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
