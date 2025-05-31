use proconio::{input, marker::Chars};

const DX: [i32; 4] = [1, 1, 0, -1];
const DY: [i32; 4] = [0, 1, 1, 1];

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    for i in 0..n {
        for j in 0..n {
            for dir in 0..4 {
                let mut x = i as i32;
                let mut y = j as i32;
                let mut cnt = 0;
                let mut black_cnt = 0;

                while x >= 0 && x < n as i32 && y >= 0 && y < n as i32 && cnt < 6 {
                    if s[x as usize][y as usize] == '#' {
                        black_cnt += 1;
                    }
                    x += DX[dir];
                    y += DY[dir];
                    cnt += 1;
                }

                if cnt == 6 && black_cnt >= 4 {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
