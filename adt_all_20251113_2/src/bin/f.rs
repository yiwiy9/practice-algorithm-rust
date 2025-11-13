use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let dx = [0, 1, 1, 1];
    let dy = [1, 1, 0, -1];

    for i in 0..n {
        for j in 0..n {
            for dir in 0..4 {
                let mut x = i;
                let mut y = j;
                let mut ok = true;
                let mut white_cnt = if s[x][y] == '.' { 1 } else { 0 };
                for _ in 0..5 {
                    let nx = x as i32 + dx[dir];
                    let ny = y as i32 + dy[dir];
                    if nx < 0 || n as i32 <= nx || ny < 0 || n as i32 <= ny {
                        ok = false;
                        break;
                    }
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if s[nx][ny] == '.' {
                        white_cnt += 1;
                        if white_cnt == 3 {
                            ok = false;
                            break;
                        }
                    }
                    x = nx;
                    y = ny;
                }

                if ok {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
