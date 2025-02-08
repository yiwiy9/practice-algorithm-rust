use proconio::{input, marker::Chars};

const dx: [i32; 8] = [0, 1, 1, 1, 0, -1, -1, -1];
const dy: [i32; 8] = [1, 1, 0, -1, -1, -1, 0, 1];

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            for dir in 0..8 {
                let mut x = i as i32;
                let mut y = j as i32;
                let mut cur = a[i][j] as usize - '0' as usize;
                for _ in 1..n {
                    x = (x + dx[dir] + n as i32) % n as i32;
                    y = (y + dy[dir] + n as i32) % n as i32;
                    cur = cur * 10 + a[x as usize][y as usize] as usize - '0' as usize;
                }
                ans = ans.max(cur);
            }
        }
    }

    println!("{}", ans);
}
