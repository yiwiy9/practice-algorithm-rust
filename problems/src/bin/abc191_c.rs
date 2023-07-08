use proconio::{input, marker::Chars};

const DX: [i32; 3] = [1, 0, 1];
const DY: [i32; 3] = [0, 1, 1];

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut ans = 0;
    for (x, s_row) in s.iter().enumerate() {
        for (y, &c) in s_row.iter().enumerate() {
            let mut black_cnt = 0;
            if c == '#' {
                black_cnt += 1;
            }

            for dir in 0..3 {
                let nx = x as i32 + DX[dir];
                let ny = y as i32 + DY[dir];

                if nx < 0 || h as i32 <= nx || ny < 0 || w as i32 <= ny {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;

                if s[nx][ny] == '#' {
                    black_cnt += 1;
                }
            }

            if black_cnt == 1 || black_cnt == 3 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
